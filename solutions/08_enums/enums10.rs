// Solution: Adding type annotations to clarify the type of value wrapped in `Some` and `None`.
fn main() {
    let some_number: Option<i8> = Some(5); // `Option<i8>` wraps an 8-bit integer.
    let some_char: Option<char> = Some('e'); // `Option<char>` wraps a character.

    let absent_number: Option<i32> = None; // Explicitly define the type of the `None` variant as `Option<i32>`.
}
// Context: In Rust, the Option enum is used to represent a value that can either exist (Some) or be absent (None). However, when using Option, Rust requires you to specify the type of the value that might be present. This helps the compiler ensure type safety.
