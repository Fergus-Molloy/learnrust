enum ListInft {
    Cons(i32, ListInft),
    Nil,
}
enum ListAllowed {
    Cons(i32, Box<ListAllowed>),
    Nil,
}

fn main() {
    println!("Hello, world!");
}
