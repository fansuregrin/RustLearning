use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Sending Multiple Values and Seeing the Receiver Waiting
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // The spawned thread will send multiple messages
        // and pause for a second between each message.
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // The main thread will wait to receive values from the spawned thread.
    for received in rx {
        println!("Got: {}", received);
    }
}
