pub struct Person  {
    name: &str,
}

impl  Person {
    pub fn new(name: & str) -> Person {
        Person{name}
    }

    pub fn greet(&self) {
        println!("Hello, my name is {}", &self.name);
    }

}