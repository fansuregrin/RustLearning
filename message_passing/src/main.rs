use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // The `send` function takes ownership of its parameter,
        // and when the value is moved, the receiver takes ownership of it.
        
        // this line will cause error: value borrowed here after move
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
