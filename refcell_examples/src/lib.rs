pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where
    T: Messager,
{
    pub fn new(
        messager: &'a T,
        max: usize,
    ) -> LimitTracker<'a, T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager.send("Warning: You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::cell::RefCell;

    struct MockMessager {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                // Using `RefCell<T>` to mutate an inner value
                // while the outer value is considered immutable
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, msg: &str) {
            // We call `borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages`
            // to get a mutable reference to the value inside the `RefCell<Vec<String>>`,
            // which is the vector.
            // Then we can call `push` on the mutable reference to the vector
            // to keep track of the messages sent during the test.
            self.sent_messages.borrow_mut().push(String::from(msg));
            // The `borrow_mut` returns the smart pointer type `RefMut<T>`,
            // which implements `Deref` trait, so it can be treated as regular reference.

            // `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.
            // If we try to violate these rules, rather than getting a compiler error
            // as we would with references, the implementation of `RefCell<T>` will **panic** at runtime.

            // This makes two mutable references in the same scope, which isn’t allowed.
            // This code will compile without any errors, but will panic at runtime.
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();  // 'already borrowed: BorrowMutError'
            one_borrow.push(String::from(msg));
            two_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);

        // We call `borrow` on the `RefCell<Vec<String>>` to get an immutable reference to the vector.
        // The `borrow` returns `Ref<T>`, which implements `Deref` trait.
        assert_eq!(1, mock_messager.sent_messages.borrow().len());
    }
}