use std::ops::Deref;
use crate::List::{Cons, Nil};

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Following the Pointer to the Value
    let x = 5;
    let y = &x;  // regular reference, a type of pointer
    assert_eq!(5, x);
    assert_eq!(5, *y); // use *dereference* operator to follow the reference to the value

    // Using Box<T> Like a Reference
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);  // dereference can be apply to `Box<T>`

    // Using MyBox<T> Like a Reference
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Implicit Deref Coercions with Functions and Methods
    let m = MyBox::new(String::from("Rust"));
    // Here we’re calling the `hello` function with the argument `&m`,
    // which is a reference to a `MyBox<String>` value.
    // Because we implemented the `Deref` trait on `MyBox<T>`, Rust can
    // turn `&MyBox<String>` into `&String` by calling `deref`.
    // The standard library provides an implementation of `Deref`
    // on `String` that returns a string slice.
    // Therefore, Rust calls `deref` again to turn the `&String` into `&str`,
    // which matches the hello function’s definition.
    hello(&m);
    // hello(&(*m)[..]);  // The code we would have to write if Rust didn’t have deref coercion

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    //   - From `&T` to `&U` when `T: Deref<Target=U>`
    //   - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
    //   - From `&mut T` to `&U` when `T: Deref<Target=U>`
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

// Defining Our Own Smart Pointer
// To enable dereferencing with the * operator on `MyBox<T>`,
// we should implement the `Deref` trait for it.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
