// Each file in the tests directory is a separate crate,
// so we need to bring our library into each test crate’s scope.
use auto_test;

mod common;

// `#[cfg(test)]` is not needed.
// Cargo treats the tests directory specially and compiles files in this directory only when we run `cargo test`.
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, auto_test::add_two(2));
}

// To run all the tests in a particular integration test file,
// use the `--test` argument of cargo test followed by **the name of the file**.

// We can’t create integration tests in the tests directory and bring functions
// defined in the src/main.rs file into scope with a use statement.