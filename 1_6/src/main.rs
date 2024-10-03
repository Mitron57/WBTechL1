use std::sync::{mpsc, atomic::{AtomicBool, Ordering}, Arc};
use std::time::Duration;

//Можно использовать другие примитивы синхронизации

fn main() {
    let timeout = Duration::from_secs(2);
    let (sx, rx) = mpsc::channel();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();
    std::thread::spawn(move || {
        while let Ok(s) = rx.recv() {
            if stop_clone.load(Ordering::Acquire) {
                return;
            }
            println!("Got: {}", s);
        }
    });
    std::thread::spawn(move || {
        std::thread::sleep(timeout);
        stop.store(true, Ordering::Release);
    });
    let mut i = 0;
    while sx.send(i).is_ok() {
        i += 1;
    }
}
