// Fix the code to manage the slice and mutation of the string safely.
// The goal is to ensure `s` and its slices are used in a way that adheres to Rust's borrowing rules.
fn main() {
    let mut s = String::from("hello"); // Create a mutable String.
    let hello: &str = &s[0..5]; // Create an immutable slice of `s`.
    println!("{hello}"); // Print the slice.
    s.push_str(" world"); // ERROR: Cannot mutate `s` while the slice `hello` is active.
}
