use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let leaf = Rc::new(
        Node {
            value: 10,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    );

    println!("leaf's parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(
        Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        }
    );

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf's parent: {:?}", leaf.parent.borrow().upgrade());

    visualize_count();
}

#[derive(Debug)]
#[allow(unused)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn visualize_count() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
