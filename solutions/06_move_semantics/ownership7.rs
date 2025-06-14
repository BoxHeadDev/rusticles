// Solution:
fn main() {
    // Solution 1: Avoid ownership transfer
    let s = String::from("hello"); // Create a new `String`.
    let b = false; // Set the condition.
    if b {
        // Do something with `s` or `s2`, but do not transfer ownership.
        println!("{}", s); // Safely use `s` as its ownership is not transferred.
    }
    println!("{}", s); // `s` is always valid here.

    // Solution 2: Use Cloning for Safety
    let s = String::from("hello"); // Create a new `String`.
    let mut s2 = String::new(); // Initialise `s2` to avoid ambiguity.
    let b = false; // Set the condition.
    if b {
        s2 = s.clone(); // Clone `s` to ensure `s` remains valid.
    }
    println!("{}", s); // Safely use `s`.
    if b {
        println!("{}", s2); // Safely use `s2` if assigned.
    }

    // Solution 3: Reassign Ownership Unconditionally
    let s = String::from("hello"); // Create a new `String`.
    let s2 = if false {
        s // Move ownership of `s` if the condition is true.
    } else {
        String::from("") // Provide a fallback `String` when the condition is false.
    };
    println!("{}", s2); // Safely use `s2`, now fully assigned.
}
// Context: The original code attempts to move ownership of s into s2 inside a conditional block. However, since the condition is false, s2 is not assigned, and s is used afterward. Rust ensures that variables involved in such ownership transfers cannot be accessed if there’s ambiguity about their validity.
