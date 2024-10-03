use std::collections::HashSet;

fn all_unique(line: &str) -> bool {
    let mut occurences = HashSet::new();
    for ch in line.to_lowercase().chars() {
        // В худшем случае contains будет O(n), но чаще всего O(1), 
        // что намного эффектинее, чем поиск по вектору
        if occurences.contains(&ch) {
            return false;
        }
        occurences.insert(ch);
    }
    true
}

fn main() {
    let ascii = "Hello, World!";
    let utf8_cyrillic = "главрыб";
    let utf8_armenian = "Զաքարյան";
    let garbage = "амHNлджС123Ա_oijгу";
    println!("ASCII: \"{}\", all_unique: {}", ascii, all_unique(ascii));
    println!("UTF-8 Cyrillic: \"{}\", all_unique: {}", utf8_cyrillic, all_unique(utf8_cyrillic));
    println!("UTF-8 Armenian: \"{}\", all_unique: {}", utf8_armenian, all_unique(utf8_armenian));
    println!("Garbage: \"{}\", all_unique: {}", garbage, all_unique(garbage));
}
