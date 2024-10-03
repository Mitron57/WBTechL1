use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::ops::Deref;
use dashmap::DashMap;

fn use_std() -> Arc<Mutex<HashMap<usize, usize>>> {
    // Можно еще использовать RwLock, но в std либе он зависит от реализаций платформы 
    // и может привести к deadlock в некоторых случаях, но т.к. здесь lock только на запись,
    // поэтому разницы с Mutex будет никакой
    let map = Arc::new(Mutex::new(HashMap::new()));
    let cpus = std::thread::available_parallelism().unwrap().get();
    let mut pool = Vec::with_capacity(cpus);
    for i in 0..cpus {
        let map = map.clone();
        pool.push(std::thread::spawn(move || {
            let i = cpus * i;
            let mut map = map.lock().unwrap();
            for elem in i..i + 10 {
                map.insert(elem, elem);
            }
        }));
    }
    while pool.iter().any(|worker| !worker.is_finished()) {}
    map
}

fn use_dashmap() -> Arc<DashMap<usize, usize>> {
    let map = Arc::new(DashMap::new());
    let cpus = std::thread::available_parallelism().unwrap().get();
    let mut pool = Vec::with_capacity(cpus);
    for i in 0..cpus {
        let map = map.clone();
        pool.push(std::thread::spawn(move || {
            let i = cpus * i;
            for elem in i..i + 10 {
                map.insert(elem, elem);
            }
        }));
    }
    while pool.iter().any(|worker| !worker.is_finished()) {}
    map
}

fn main() {
    let hash_map = use_std();
    let dash_map = use_dashmap();
    println!("HashMap: {:?}", hash_map.lock().unwrap().deref());
    println!("DashMap: {:?}", dash_map.deref());
}
