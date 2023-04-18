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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_will_fail() {
        panic!("Make this test fail.")
    }

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
}
