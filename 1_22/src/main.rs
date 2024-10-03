unsafe fn remove_unsafe<T>(vec: &mut Vec<T>, i: usize) {
    if i >= vec.len() {
        return;
    }
    let ptr = vec.as_ptr();
    let len = vec.len();
    if i + 1 < len {
        ptr.add(i + 1).copy_to(ptr.add(i) as *mut T, len - i - 1);
    }
    vec.set_len(len - 1);
}


fn remove<T>(vec: &mut Vec<T>, i: usize) {
    if i < vec.len() {
        vec.remove(i);
    }
}

fn main() {
    let mut vec_1 = vec![2, 5, 1, 9, 4];
    let mut vec_2 = vec![2, 5, 1, 9, 4];
    unsafe {remove_unsafe(&mut vec_1, 4)};
    remove(&mut vec_2, 4);
    assert_eq!(vec_1, vec_2);
}
