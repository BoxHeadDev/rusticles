// 🦀 Rustlings Challenge: Mocking with RefCell<T> and Interior Mutability
//
// You're building a `LimitTracker` that warns when a user exceeds 75% of a quota.
// The `Messenger` trait takes `&self` in `send`, so we can't use `&mut self`.
//
// Your job:
// 1. Store `sent_messages` in a `RefCell<Vec<String>>` inside `MockMessenger`.
// 2. Use `.borrow_mut()` in `send()` to push messages into the list.
// 3. Use `.borrow()` in the test to check how many messages were recorded.
//
// HINTS:
// - You can't change the trait definition or the `&self` method.
// - Use `RefCell` to allow mutation inside an immutable reference.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let pct = self.value as f64 / self.max as f64;

        if pct >= 0.75 {
            self.messenger.send("Warning: Over 75%");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // TODO: Change this to use RefCell
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                // TODO: Wrap the vector in a RefCell
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // TODO: Use `.borrow_mut()` to push to the vector inside RefCell
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 100);

        tracker.set_value(80);

        // TODO: Use `.borrow()` to check message count
        assert_eq!(mock.sent_messages.len(), 1);
    }
}
