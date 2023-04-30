use crate::List::{Cons, Nil};

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// This snip of code won't be compiled because
// recursive type `List` has infinite size.
//
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// `Box<T>` is a smart pointer.
// Boxes provide only the indirection and heap allocation;
// they don’t have any other special capabilities,
// like those we’ll see with the other smart pointer types. 
enum List {
    Cons(i32, Box<List>),
    Nil,
}