///Вариант 2: если я правильно понял условие, то допустим вариант с настраиваемым сообщением,
///т.е. fn say(&self, msg: &str) так же допустимо

trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let me = Person{name: String::from("Armen Zakharyan")};
    me.say();
}
