// Add type annotations to variables that use the Option enum to clarify their intended types.
fn main() {
    let some_number = Some(5); // Hint: Specify the type of the number wrapped in `Some`.
    let some_char = Some('e'); // Hint: Specify the type of the character wrapped in `Some`.

    let absent_number = None; // Hint: Specify the type of the `Option` explicitly.
}
