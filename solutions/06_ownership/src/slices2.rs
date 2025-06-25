// Solution:
fn main() {
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
