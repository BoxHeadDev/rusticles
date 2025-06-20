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
    // TODO: Return Ok(a / b) or Err(...) if b == 0
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_normally() -> Result<(), String> {
        // TODO: Call safe_divide with non-zero denominator and use `?` to propagate errors
        todo!()
    }

    #[test]
    fn division_by_zero_fails() {
        // TODO: Call safe_divide with 0 and assert it returns an Err
        todo!()
    }
}
