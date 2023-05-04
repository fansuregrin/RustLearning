use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Creating Multiple Producers by Cloning the Transmitter
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("the"),
            String::from("spice"),
            String::from("might"),
            String::from("flow"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("the"),
            String::from("boy"),
            String::from("who"),
            String::from("lived"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    // The order of each run may be different.
    for received in rx {
        println!("Got: {}", received);
    }
    // On my machine, the output of first run is:
    // ```
    // Got: the
    // Got: the
    // Got: spice
    // Got: might
    // Got: boy
    // Got: flow
    // Got: who
    // Got: lived
    // ```
    // the result of second run is:
    // ```
    // Got: the
    // Got: the
    // Got: spice
    // Got: boy
    // Got: might
    // Got: flow
    // Got: who
    // Got: lived
    // ```
}
