use std::thread;
use std::time::Duration;

fn main() {
    // Creating a New Thread with `spawn`
    // The return type of `thread::spawn` is `JoinHandle`.
    // A `JoinHandle` is an owned value that, when we call the `join`
    // method on it, will wait for its thread to finish.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            // The calls to `thread::sleep` force a thread to stop its execution for a short duration.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();
    // 
    // If you call `join` here, the output may like this:
    // ```
    // hi number 1 from the spawned thread!
    // hi number 2 from the spawned thread!
    // hi number 3 from the spawned thread!
    // hi number 4 from the spawned thread!
    // hi number 5 from the spawned thread!
    // hi number 6 from the spawned thread!
    // hi number 7 from the spawned thread!
    // hi number 8 from the spawned thread!
    // hi number 9 from the spawned thread!
    // hi number 1 from the main thread!
    // hi number 2 from the main thread!
    // hi number 3 from the main thread!
    // hi number 4 from the main thread!
    // ```

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Calling `join` on the handle blocks the thread currently running
    // until the thread represented by the handle terminates.
    handle.join().unwrap();
    // 
    // the output may like this:
    // ```
    // hi number 1 from the main thread!
    // hi number 1 from the spawned thread!
    // hi number 2 from the spawned thread!
    // hi number 2 from the main thread!
    // hi number 3 from the main thread!
    // hi number 3 from the spawned thread!
    // hi number 4 from the spawned thread!
    // hi number 4 from the main thread!
    // hi number 5 from the spawned thread!
    // hi number 6 from the spawned thread!
    // hi number 7 from the spawned thread!
    // hi number 8 from the spawned thread!
    // hi number 9 from the spawned thread!
    // ```

    let v = vec![1, 2, 3];

    // This snip of code won't compile!!!
    // Rust can’t tell how long the spawned thread will run,
    // so it doesn’t know if the reference to `v` will always be valid.
    // 
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // By adding the `move` keyword before the closure, we force the closure to take ownership
    // of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    // 
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
