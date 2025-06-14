// Optimise the code by reducing redundant slicing operations.
// The goal is to explore and understand string slicing in Rust.
fn main() {
    let s = String::from("hello"); // Create a string.

    let len = s.len(); // Get the length of the string.

    // Reference part of the string.
    let slice = &s[3..len]; // Slice from index 3 to the end of the string.
    let slice = &s[3..]; // Simplify: Omit the `len` in slicing.

    // Reference the whole string.
    let slice = &s[0..len]; // Slice from index 0 to the end.
    let slice = &s[..]; // Simplify: Use the shorthand for the whole string.
}
