// Fix the code to handle conditional ownership transfer safely.
// The goal is to ensure `s` and `s2` are properly managed without ambiguity.
fn main() {
    let s = String::from("hello"); // Create a new `String`.
    let s2; // Declare `s2` without initialisation.
    let b = false; // Set the condition.
    if b {
        s2 = s; // Move ownership of `s` to `s2` if `b` is true.
    }
    println!("{}", s); // ERROR: `s` might have been moved, so this is not allowed.
}
