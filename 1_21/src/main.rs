use num_bigint::BigInt;

// В большинстве случаев хватит и репрезентации из f64, 
// но если нужно не терять точность, то BigInt - один из лучших вариантов

fn main() {
    let a = BigInt::from(2).pow(50);
    let b = BigInt::from(3).pow(60);
    
    let division = &a / &b;
    let multiplication = &a * &b;
    let addition = &a + &b;
    let subtraction = &a - &b;
    
    println!("BigInt:");
    
    println!("{} / {} = {}", a, b, division);
    println!("{} + {} = {}", a, b, addition);
    println!("{} - {} = {}", a, b, subtraction);
    println!("{} * {} = {}", a, b, multiplication);
    
    println!("Comparison with f64:");
    
    //пример с f64
    let a = 2f64.powf(50.0);
    let b = 3f64.powf(60.0);
    println!("{} * {} = {}", a, b, a * b);
}
