use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{RecvError, SendError};

pub fn channel<T: Send>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(
        Inner {
            queue: Mutex::new(VecDeque::new()),
            alive_receivers: AtomicU64::new(1),
            closed: AtomicBool::new(false),
            notifier: Condvar::new(),
        }
    );
    let receiver = Receiver { inner: inner.clone() };
    let sender = Sender { inner };
    (sender, receiver)
}


struct Inner<T: Send> {
    queue: Mutex<VecDeque<T>>,
    alive_receivers: AtomicU64, //count all Receivers, used for preventing sending on disconnected sender
    closed: AtomicBool, //when Sender dropped
    notifier: Condvar, //notify threads on every action (e.g. send, or drop on the sender)
}

pub struct Sender<T: Send> {
    inner: Arc<Inner<T>>,
}

impl<T: Send> Sender<T> {
    pub fn send(&self, item: T) -> Result<(), SendError<T>> {
        if self.inner.alive_receivers.load(Ordering::Acquire) == 0 {
            return Err(SendError(item));
        }
        self.inner.queue.lock().unwrap().push_back(item);
        self.inner.notifier.notify_one();
        Ok(())
    }
}

impl<T: Send> Drop for Sender<T> {
    fn drop(&mut self) {
        self.inner.closed.store(true, Ordering::Release);
        self.inner.notifier.notify_all();
    }
}

pub struct Receiver<T: Send> {
    inner: Arc<Inner<T>>,
}

impl<T: Send> Receiver<T> {
    pub fn recv(&self) -> Result<T, RecvError> {
        let mut queue = self.inner.queue.lock().unwrap();
        while queue.is_empty() {
            if self.inner.closed.load(Ordering::Acquire) {
                return Err(RecvError);
            }
            queue = self.inner.notifier.wait(queue).unwrap();
        }
        Ok(queue.pop_front().unwrap())
    }
}

impl<T: Send> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        self.inner.alive_receivers.fetch_add(1, Ordering::Release);
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Send> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.inner.alive_receivers.fetch_sub(1, Ordering::Release);
    }
}