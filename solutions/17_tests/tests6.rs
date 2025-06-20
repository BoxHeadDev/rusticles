// 🦀 Rustlings Challenge: Testing Equality with assert_eq! and assert_ne!
//
// ✅ Implement a function called `add_two` that returns the input plus 2.
// ✅ Write one test that uses `assert_eq!` to verify `add_two(3)` returns `5`.
// ✅ Write another test that uses `assert_ne!` to assert that `add_two(3)` is NOT equal to `4`.
//
// TIP: These macros print both values on failure, so they're great for debugging.
// NOTE: Your function must return the correct value — don't introduce bugs on purpose here.

pub fn add_two(x: usize) -> usize {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_correctly() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn does_not_return_four_for_three() {
        let result = add_two(3);
        assert_ne!(result, 4);
    }
}
