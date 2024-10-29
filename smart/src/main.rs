
mod first;
use first::find_longest;

fn main() {

    let s1 = String::from("Hello");
    let s2 = String::from("world!");

    let result = find_longest(&s1, &s2);
    
    println!("longest string: {}", result);
}
