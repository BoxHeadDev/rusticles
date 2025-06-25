// 🦀 Rustlings Challenge: Adding Custom Failure Messages
//
// ✅ Implement a function called `greeting` that returns a string greeting a user by name.
// ✅ Write a test that asserts the output contains the name.
// ✅ Improve the test with a custom failure message that shows the actual output.
//
// 📌 TIP: Use `format!()` to build the greeting and to format your custom failure message.
// ❌ Do NOT use assert_eq! — use assert!(condition, "...") with a helpful custom message.

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
