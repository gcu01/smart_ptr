pub struct Person <'a> {
    name: &'a str,
}

impl <'a> Person<'a> {
    pub fn new(name: &'a str) -> Person<'a> {
        Person{name}
    }

    pub fn greet(&self) {
        println!("Hello, my name is {}", &self.name);
    }

}