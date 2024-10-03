use std::ops::Range;
use std::sync::Arc;
use std::thread::{self, JoinHandle, available_parallelism};

fn generate_array(size: usize) -> Vec<usize> {
    (1..=size).collect()
}

fn start_thread(array: Arc<Vec<usize>>, range: Range<usize>) -> JoinHandle<()> {
    thread::spawn(move || {
        for elem in &array[range] {
            println!("{}", elem*elem);
        }
    })
}

// Вариант 2: создать mpsc::channel, спавнить потоки, 
// мувать в них клоны sender'ов и ждать через receiver в основном потоке

/// size: N в числовом ряду 1..N, 
/// chunk_size: количество чисел, которые будут обрабатываться в одном потоке, 
/// если значение 0 или превышет размер массива, 
/// то размер чанка определяется относительно доступных логических ядер процессора
fn print_squares(size: usize, mut chunk_size: usize) {
    let array = Arc::new(generate_array(size));
    let cpus = available_parallelism().unwrap().get();
    if chunk_size == 0 || size < chunk_size {
        chunk_size = size / cpus;
    }
    let mut pool = Vec::with_capacity(size / chunk_size + 1);
    for i in 0..=size / chunk_size - 1 {
        let arr = array.clone();
        pool.push(start_thread(arr, i * chunk_size..(i + 1) * chunk_size));
    }
    if size % chunk_size != 0 {
        pool.push(start_thread(array, chunk_size * (size / chunk_size)..size));
    }
    while pool.iter().any(|worker| !worker.is_finished()) {}
}

fn main() {
    print_squares(1000, 0);
}
