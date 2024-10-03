use std::fmt::Display;

trait Publishable { //Наш интерфейс
    type Article: Display;
    
    fn publish(&self) -> Self::Article;
}

struct Magazine;

impl Magazine { //Журнал, который ждет издателя
    fn release_article(&self, publisher: &impl Publishable)
    {
        println!("{}", publisher.publish());
    }
}

struct Author;

impl Author { //Творец, но что он может в этом мире без издателя?
    fn write_article(&self) -> &str {
        "*some scientific stuff*"
    }
}

// Необязательно хранить ссылкой, зависит от условий задачи, 
// нужен ли нам interior mutability, или может вообще наш адаптер должен быть thread-safe
struct Publisher<'a> {
    author: &'a Author,
}

impl<'a> Publishable for Publisher<'a> { //Адаптируем нашего писателя к миру научных статей
    type Article = &'a str;
    
    fn publish(&self) -> Self::Article {
        self.author.write_article()
    }
}

fn main() {
    let universum = Magazine;
    let student = Author;
    let university = Publisher { author: &student };
    universum.release_article(&university);
}
