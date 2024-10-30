
mod first;
use first::find_longest;

mod second;
use second::Person;

mod third;
use third::{Author,Book};

fn main() {

    let s1 = String::from("Hello");
    let s2 = String::from("world!");

    let result = find_longest(&s1, &s2);
    
    println!("longest string: {}", result);

    let name = String::from("Allice");
    let person = Person::new(&name);
    person.greet();

    let author_name = "Maya Angelou";
    let author :Author = Author::new(&author_name);
    let book_title = "Bird's cage";
    let book = Book::new(&book_title, author, 2024);
    book.display();

    let author_name2 = "Anghel Saligni";
    let author2: Author = Author::new(&author_name2);
    let book_title2 = "The bridge";
    let book2 = Book::new(&book_title2, author2, 2021);
    book2.display();

}
