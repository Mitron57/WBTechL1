use std::fmt::Display;
use std::thread::JoinHandle;

mod channel;

fn start_thread<T: Send + Display + 'static>(rx: channel::Receiver<T>) -> JoinHandle<()>
{
    std::thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            let id = std::thread::current().id();
            println!("Thread: {:?}, Got: {}", id, msg);
        }
    })
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut cpus = std::thread::available_parallelism().unwrap().get();
    if args.len() == 3 && args[1] == "--workers" {
        match args[2].parse::<usize>() {
            Ok(count) => {
                cpus = count;
                println!("Using user-defined worker pool: {} threads", cpus);
            }
            Err(_) => {
                println!("Error: Invalid number of threads");
                return;
            }
        }
    } else {
        println!("Using standard worker pool (cpu count): {} threads", cpus);
    }
    let (tx, rx) = channel::channel();
    let mut pool = Vec::with_capacity(cpus); //worker pool
    for _ in 0..cpus {
        pool.push(start_thread(rx.clone()));
    }
    let mut data = 0;
    while tx.send(data).is_ok() {
        data += 1;
        if data == 100 {
            drop(tx);
            break;
        }
    }
    //hold up, let them cook 🔥 (wait all workers)
    while pool.iter().any(|worker| !worker.is_finished()) {}
}
