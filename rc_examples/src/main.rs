use std::rc::Rc;
use crate::List::{Cons, Nil};

#[allow(unused)]
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));
}

// If we define `List` like this:
// ```
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
// ```
// 
// This snip of code won't be compiled.
// ```
// let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
// let b = Cons(3, Box::new(a));
// let c = Cons(4, Box::new(a));
// ```
// The `Cons` variants own the data they hold,
// so when we create the `b` list, `a` is moved into `b` and `b` owns `a`.
// Then, when we try to use `a` again when creating `c`,
// we’re not allowed to because `a` has been moved.

// Changing definition of `List` to use `Rc<T>` in place of `Box<T>`
enum List {
    Cons(i32, Rc<List>),
    Nil,
}