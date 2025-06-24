// 🦀 Rustlings Challenge: Checking for Panics with should_panic
//
// ✅ Implement a `Guess` struct with a `new` function that panics
//    if the value is not in the range 1..=100.
// ✅ Write two tests:
//   - One that expects a panic for values > 100
//   - One that uses `#[should_panic(expected = "...")]` to test the message
//
// ❗ Do NOT try to catch panics manually — the attribute handles it for you.

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be >= 1, got {value}");
        } else if value > 100 {
            panic!("Guess value must be <= 100, got {value}");
        }

        Guess { value }
    }
}

// TODO: Write a test module with two test functions:
// 1. `#[should_panic]` test for value 200
// 2. `#[should_panic(expected = "...")]` test with expected substring
