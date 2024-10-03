use std::ops::{Mul, Range};
use std::sync::{mpsc, Arc};
use std::thread::available_parallelism;

fn generate_array(size: u128) -> Vec<u128> {
    (1..=size).collect()
}

fn start_thread<T: Send + Sync + 'static>(sender: mpsc::Sender<T>, array: Arc<Vec<T>>, range: Range<usize>)
where
        for<'a> &'a T: Mul<&'a T, Output=T>, //Для любого 'a &T должен удовлетворять этим ограничениям
{
    std::thread::spawn(move || {
        for elem in &array[range] {
            sender.send(elem * elem).unwrap();
        }
    });
}

//Есть второй варинат, но он не совсем по заданию: 
//сумму обернуть в Mutex, и суммировать сразу в нее во всех потоках
fn sum_squares(size: usize, mut chunk_size: usize) -> u128 {
    let array = Arc::new(generate_array(size as u128));
    let mut sum = 0;
    let cpus = available_parallelism().unwrap().get();
    if chunk_size == 0 || size < chunk_size {
        chunk_size = size / cpus; //Разделим данные на чанки
    }
    let (sender, receiver) = mpsc::channel();
    for i in 0..size / chunk_size {
        let sx = sender.clone();
        let arr = array.clone();
        start_thread(sx, arr, i * chunk_size..(i + 1) * chunk_size);
    }
    if size % chunk_size != 0 {
        start_thread(sender, array, chunk_size * (size / chunk_size)..size);
    } else {
        drop(sender);
    }
    while let Ok(square) = receiver.recv() {
        sum += square;
    }
    sum
}

fn main() {
    println!("{}", sum_squares(10000000, 0));
}
