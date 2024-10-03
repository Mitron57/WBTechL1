fn reverse_chars(s: &str) -> String {
    s.chars().rev().collect()
}
fn main() {
    let line = "главрыба";
    println!("Origin: {}", line);
    println!("Reversed: {}", reverse_chars(line));
    println!("Reverse reversed {}", reverse_chars(&reverse_chars(line)));
}
