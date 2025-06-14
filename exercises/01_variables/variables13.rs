// Fix the code to use shadowing instead of mutability
fn main() {
    let mut spaces = "   "; // This variable is mutable, but we don't need mutability here
    spaces = spaces.len(); // Transforming `spaces` from a string to its length causes a type mismatch
}
