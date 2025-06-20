// 🦀 Rustlings Challenge: Using Result<T, E> in Tests
//
// ✅ Implement a function called `safe_divide` that divides two numbers.
//    It should return `Ok(result)` if the denominator is not 0,
//    and `Err("Cannot divide by zero")` otherwise.
//
// ✅ Write a test using `Result<(), String>` instead of `assert!` macros.
// ✅ Use the `?` operator to propagate errors cleanly.
//
// 🔁 HINT: To test that division by zero fails, use `assert!(value.is_err())`.
// ⚠️ You can't use #[should_panic] with `Result`-based tests.

pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_normally() -> Result<(), String> {
        let result = safe_divide(10, 2)?;
        if result == 5 {
            Ok(())
        } else {
            Err(format!("Expected 5, got {}", result))
        }
    }

    #[test]
    fn division_by_zero_fails() {
        let result = safe_divide(10, 0);
        assert!(result.is_err(), "Expected an error, but got: {:?}", result);
    }
}
