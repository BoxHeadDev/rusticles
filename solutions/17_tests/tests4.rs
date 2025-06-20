// 🦀 Rustlings Challenge: Write Tests From Scratch
//
// Your job is to write a complete test module from scratch.
//
// 1. Create a test module using `#[cfg(test)] mod tests { ... }`.
// 2. Inside the module, `use super::*` to access the outer functions.
// 3. Write one test function that verifies `multiply(2, 3)` returns 6 using `assert_eq!`.
// 4. Write another test function that *always fails* by calling `panic!` with a message.
//
// Once you're done, run `cargo test` and observe both test outcomes.

pub fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication_works() {
        let result = multiply(2, 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn this_test_fails_on_purpose() {
        panic!("This test always fails!");
    }
}
