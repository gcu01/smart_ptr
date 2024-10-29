
mod first;
use first::find_longest;

mod second;
use second::Person;

fn main() {

    let s1 = String::from("Hello");
    let s2 = String::from("world!");

    let result = find_longest(&s1, &s2);
    
    println!("longest string: {}", result);

    let name = String::from("Allice");
    let person = Person::new(&name);
    person.greet();
}
