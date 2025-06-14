// Solution: The issue can be fixed by returning a string slice (&str) instead of an index, which ties the word to the String and prevents modifying the String while the slice is borrowed.
fn bug_example() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // Get a string slice representing the first word.
                               // s.clear(); // ERROR: Cannot modify `s` while `word` exists.
    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // Return a slice of the string up to the first space.
        }
    }
    &s[..] // If no space is found, return the entire string.
}
// Context: The original code demonstrates a common pitfall: storing the result of a function (first_word) that returns an index into a String, then modifying the String. This leads to a disconnect between the String and its associated index, which Rust does not automatically track.

// Solution:
fn slice() {
    // Solution 1: Drop the Slice Before Mutation
    let mut s = String::from("hello"); // Create a mutable String.
    {
        let hello: &str = &s[0..5]; // Create an immutable slice of `s`.
        println!("{hello}"); // Use the slice within a restricted scope.
    } // The slice `hello` goes out of scope here.
    s.push_str(" world"); // Safely mutate `s` after the slice is dropped.
    println!("{s}"); // Print the modified String.

    // Solution 2: Clone the Slice
    let mut s = String::from("hello"); // Create a mutable String.
    let hello: String = s[0..5].to_string(); // Clone the slice into a new String.
    println!("{hello}"); // Print the cloned slice.
    s.push_str(" world"); // Safely mutate `s` because `hello` is now independent.
    println!("{s}"); // Print the modified String.
}
// Context: The original code creates an immutable slice (hello) of the string s, then tries to mutate s by appending to it. Rust prevents this because you cannot mutate a String while an immutable slice is still active.

// Solution: The code is already valid, but redundant slicing operations can be removed for clarity.
fn range() {
    let s = String::from("hello"); // Create a string.

    // Reference part of the string.
    let slice = &s[3..]; // Slice from index 3 to the end of the string.

    // Reference the whole string.
    let slice = &s[..]; // Slice the entire string.
}
// Context: The code demonstrates how to create slices of a String for referencing parts or the whole string.

fn main() {
    bug_example();
    slice();
    range();
}
