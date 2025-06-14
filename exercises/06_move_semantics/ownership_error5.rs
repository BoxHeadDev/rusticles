// Fix the code to respect Rust's borrowing rules around mutable references.
// The goal is to ensure there are no conflicting or invalid references.
fn main() {
    let mut n = 0;
    let a = &mut n; // Create a mutable reference to `n`.
    let b = a; // ERROR: This transfers ownership of the mutable reference from `a` to `b`.
}
