// Fix the code to prevent modifying the string while using its word reference.
// The goal is to link the string and its first word more safely.
fn bug_example() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // Get the index of the first word.
    s.clear(); // ERROR: The string can be modified, making the index invalid.
    println!("The first word is at index: {}", word);
}

fn first_word(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// Fix the code to manage the slice and mutation of the string safely.
// The goal is to ensure `s` and its slices are used in a way that adheres to Rust's borrowing rules.
fn slice() {
    let mut s = String::from("hello"); // Create a mutable String.
    let hello: &str = &s[0..5]; // Create an immutable slice of `s`.
    println!("{hello}"); // Print the slice.
    s.push_str(" world"); // ERROR: Cannot mutate `s` while the slice `hello` is active.
}

// Optimise the code by reducing redundant slicing operations.
// The goal is to explore and understand string slicing in Rust.
fn range() {
    let s = String::from("hello"); // Create a string.

    let len = s.len(); // Get the length of the string.

    // Reference part of the string.
    let slice = &s[3..len]; // Slice from index 3 to the end of the string.
    let slice = &s[3..]; // Simplify: Omit the `len` in slicing.

    // Reference the whole string.
    let slice = &s[0..len]; // Slice from index 0 to the end.
    let slice = &s[..]; // Simplify: Use the shorthand for the whole string.
}

fn main() {
    bug_example();
    slice();
    range();
}
