use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Counter {
    count: Arc<Mutex<usize>>,
}

impl Counter {
    fn new(base: usize) -> Counter {
        Counter {
            count: Arc::new(Mutex::new(base)),
        }
    }
    fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }
}

fn main() {
    let cpus = std::thread::available_parallelism().unwrap().get();
    let counter = Counter::new(0);
    let mut pool = Vec::with_capacity(cpus);
    for _ in 0..cpus {
        let counter = counter.clone();
        pool.push(std::thread::spawn(move || {
            for _ in 0..10 {
                counter.increment();
            }
        }));
    }
    //wait for them
    while pool.iter().any(|worker| !worker.is_finished()) {}
    println!("{}", counter.count.lock().unwrap());
}
