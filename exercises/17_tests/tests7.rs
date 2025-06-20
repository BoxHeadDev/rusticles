// 🦀 Rustlings Challenge: Adding Custom Failure Messages
//
// ✅ Implement a function called `greeting` that returns a string greeting a user by name.
// ✅ Write a test that asserts the output contains the name.
// ✅ Improve the test with a custom failure message that shows the actual output.
//
// 📌 TIP: Use `format!()` to build the greeting and to format your custom failure message.
// ❌ Do NOT use assert_eq! — use assert!(condition, "...") with a helpful custom message.

pub fn greeting(name: &str) -> String {
    // TODO: Return a string like "Hello {name}!"
    String::from("Hello!") // ← This will cause the test to fail initially
}

// TODO: Write your test module and test function here
// 1. Use `#[cfg(test)]` and `#[test]`
// 2. Use `assert!(...)` with a helpful message like:
//    "Greeting did not contain name, value was `{}`", result
