use std::collections::VecDeque;

fn reverse_deque_variant(s: &str) -> String {
    let mut stack = VecDeque::new();
    for word in s.split_whitespace() {
        stack.push_front(word);
    }
    let mut reversed = String::new();
    while stack.len() > 1 {
        let top = stack.pop_front().unwrap();
        reversed.push_str((top.to_owned() + " ").as_str());
    }
    reversed + stack.pop_front().unwrap()
}

//Используем магию ZCA
fn reverse_by_space(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

fn main() {
    let line = "snow dog sun";
    println!("reverse_by_space: {}", reverse_by_space(line));
    println!("reverse_deque_variant: {}", reverse_deque_variant(line));
}
