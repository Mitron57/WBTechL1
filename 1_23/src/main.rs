mod point;

use point::Point;

fn main() {
    let a = Point::new(-10.0, 10.0);
    let b = Point::new(13.0, 14.0);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("Distance from a to b: {}", a.distance_to(&b));
    println!("Distance from b to a: {}", b.distance_to(&a));
}
