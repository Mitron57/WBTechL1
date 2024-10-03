fn partition(array: &mut [i32], left: usize, right: usize) -> usize { // Функция разделения массива
    let pivot = array[right];
    let mut i = left;
    for j in left..right {
        if array[j] < pivot {
            array.swap(j, i);
            i += 1;
        }
    }
    array.swap(i, right);
    i
}

fn core(array: &mut [i32], left: usize, right: usize) { // сама сортировка
    if left < right {
        let p = partition(array, left, right);
        if p > 0 {
            core(array, left, p - 1);
        }
        if p < right {
            core(array, p + 1, right);
        }
    }
}

fn qsort(array: &mut [i32]) { //обертка 
    if array.len() < 2 {
        return;
    }
    core(array, 0, array.len() - 1);
}

fn main() {
    let mut array = [9, 8, 7, 6, 5, 4, 3, 1];
    qsort(&mut array);
    println!("{:?}", array);
}
