pub struct Author<'a> {
    name: &'a str,
}

pub struct Book<'a> {
    title: &'a str,
    author: Author<'a>,
    publication_year: i32,
}

impl<'a> Author<'a> {
    pub fn new(name: &'a str) -> Author<'a> {
        Author{name}
    }
}

impl<'a> Book<'a> {
    pub fn new(title: &'a str, author: Author<'a>, publication_year: i32) -> Book<'a>{
        Book{title, author, publication_year,}
    }

    pub fn display(&self) {
        println!("{} ({}) by {}", self.title, self.publication_year, self.author.name);
    }
}