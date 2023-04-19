pub fn add_two(n: i32) -> i32 {
    n + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(unused)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(unused)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[allow(unused)]
fn prints_and_returns_ten(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// Unit Tests
#[cfg(test)] // This line tells Rust to compile and run the test code only when you run `cargo test`.
mod tests {
    use super::*; // This line brings all of the test module’s parent’s items into scope
    // Rust’s privacy rules do allow you to test private functions

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // #[test]
    // fn it_will_fail() {
    //     panic!("Make this test fail.")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Bob");
        assert!(
            result.contains("Bob"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // This test will pass because the value we put in the should_panic attribute’s
    // expected parameter is a substring of the message that the Guess::new function panics with.
    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn less_than_1() {
        Guess::new(-1);
    }

    // using Result<T, E> in tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus to two does not equal to four!"))
        }
    }

    // By default, if a test passes, Rust’s test library captures anything printed to standard output.
    // If we want to see printed values for passing tests as well, we can tell Rust to also show
    // the output of successful tests with `--show-output`. 
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_ten(1);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_ten(3);
    //     assert_eq!(5, value);
    // }

    // We can run a subset of tests by name.
    // For example, run `cargo test it_works` to see what will happend
    // However, we cannot specify the names of multiple tests in this way;
    // only the first value given to `cargo test` will be used.

    // So, we cann use filter to run multiple tests.
    // Try to run `cargo test than`.

    // Ignoring Some Tests Unless Specifically Requested
    // If we want to run only the ignored tests, we can use `cargo test -- --ignored`.
    // If you want to run all tests whether they’re ignored or not, you can run `cargo test -- --include-ignored`.
    #[test]
    #[ignore] // this test will be ignored
    fn expensive_test() {
        // code that takes an hour to run
    }
}
