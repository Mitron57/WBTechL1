use std::collections::HashSet;

fn intersection(lhs: &HashSet<i32>, rhs: &HashSet<i32>) -> HashSet<i32> {
    let mut intersection = HashSet::new();
    for elem in lhs.iter() {
        if rhs.contains(elem) {
            intersection.insert(*elem);
        }
    }
    intersection
}

fn main() {
    let set_1 = HashSet::from([1,3, -1337, 228]);
    let set_2 = HashSet::from([0, 78, 56, 1, -1337]);
    let set_3 = set_1.intersection(&set_2);
    let set_4 = intersection(&set_1, &set_2);
    println!("Set_1: {:?}", set_1);
    println!("Set_2: {:?}", set_2);
    println!("Set_3 (std::collections::HashSet::intersection): {:?}", set_3);
    println!("Set_4 (my implementation): {:?}", set_4);
}
