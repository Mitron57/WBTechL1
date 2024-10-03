fn print_bits(num: i64) {
    for i in (0..64).rev() {
        let bit = (num >> i) & 1; //находим i-ый бит
        print!("{}", bit);
        if i % 8 == 0 {
            print!(" ");
        }
    }
    println!();
}

fn set_bit(num: &mut i64, bit_number: u8, set: bool) {
    if bit_number >= 64 {
        return;
    }
    *num = if set {
        *num | (1 << bit_number) //установка 1
    } else {
        *num & !(1 << bit_number) //установка 0
    }
}

fn main() {
    let mut num = -1i64;
    println!("before:");
    print_bits(num);
    set_bit(&mut num, 63, false);
    println!("after:");
    print_bits(num);
}
