use std::any::Any;

fn main() {
    //type erasure (hell yeah polymorphism)
    let var: &dyn Any = &String::from("hello");
    //Можно еще проверить через Any::is
    if let Some(inner) = var.downcast_ref::<String>() {
        println!("This is a String! Value: {}", inner);
    } else {
        println!("This was not a String!😭");
    }
}
