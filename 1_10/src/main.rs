use std::sync::mpsc;
use std::thread;

fn generate_array(size: usize) -> Vec<usize> {
    (1..size).collect()
}

// Вариант 2: можно сделать через tokio и его канал, 
// но мы так просто поменяем многопоточность на асинхронность, код изменится не шибко сильно

fn main() {
    let array = generate_array(10);
    let (tx_mul, rx_mul) = mpsc::channel();
    let (tx_print, rx_print) = mpsc::channel();
    thread::spawn(move || {
        while let Ok(num) = rx_mul.recv() {
            tx_print.send(num * num).unwrap();
        }
    });
    let printer = thread::spawn(move || {
        while let Ok(num) = rx_print.recv() {
            println!("{}", num);
        }
    });
    for num in array {
        tx_mul.send(num).unwrap();
    }
    drop(tx_mul);
    printer.join().unwrap();
}
