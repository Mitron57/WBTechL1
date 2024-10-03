fn binary_search(array: &[i32], value: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        //consider use match std::cmp::cmp + Ordering
        if array[mid] < value {
            left = mid + 1;
        } else if array[mid] > value {
            right = mid - 1;
        } else {
            return Some(mid);
        }
    }
    None
}

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("Array: {:?}", array);
    println!("Index of {} in array: {}", 8, binary_search(&array, 8).unwrap());
}
