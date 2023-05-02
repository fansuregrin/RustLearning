use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

fn main() {
    //  both `a` and `value` have ownership of the inner 5 value
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("Before modification.");
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    // Here appears *automatic dereferencing* feature
    // value.borrow_mut ~= (*value).borrow_mut ~= value->borrow_mut
    // The `borrow_mut` method returns a `RefMut<T>` smart pointer,
    // and we use the dereference operator on it and change the inner value.
    *value.borrow_mut() += 10;

    println!("After modification.");
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
