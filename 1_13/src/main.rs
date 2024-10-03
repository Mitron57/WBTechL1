use std::io::{stdin};
use std::hash::{DefaultHasher, Hash, Hasher};

// Наивный вариант: просто складывать строки в массив, и проверять, 
// но более эффективный вариант сверять по хешу

fn main() {
    let mut line = String::new();
    let mut hashes = Vec::new();
    while stdin().read_line(&mut line).is_ok() {
        let mut hasher = DefaultHasher::new();
        line.hash(&mut hasher);
        let hash = hasher.finish();
        if hashes.contains(&hash) {
            line.clear();
            continue;
        }
        hashes.push(hasher.finish());
        println!("{line}");
        line.clear();
    }
}
