use std::env;
use std::process;

use minigrep_improved::Config;

fn main() { 
    // If the `Result` is an `Ok` value, this method’s behavior is similar to `unwrap`:
    // it returns the inner value `Ok` is wrapping. However, if the value is an `Err` value,
    // this method calls the code in the closure, which is an anonymous function we define
    // and pass as an argument to `unwrap_or_else`.
    // 
    // Passing ownership of the iterator returned from `env::args` to `Config::build` directly.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        // The `process::exit` function will stop the program immediately
        // and return the number that was passed as the exit status code.
        process::exit(1);
    });

    if let Err(e) = minigrep_improved::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
