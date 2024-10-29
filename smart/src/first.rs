pub fn find_longest<'a> (t1: &'a str, t2: &'a str) -> &'a str {
    use std::cmp::Ordering::*;

    match t1.len().cmp(&t2.len()) {
        Greater => &t1,
        Equal => "",
        Less => &t2,
    }
}
