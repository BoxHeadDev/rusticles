// Solution: Use shadowing to create a new variable with the same name
fn main() {
    let spaces = "   "; // Declare `spaces` as a string
    let spaces = spaces.len(); // Shadow `spaces` to store its length as an integer
    println!("The length of spaces is: {spaces}"); // Prints the length of the string
}
// Context: In Rust, shadowing allows you to reuse the same variable name with a new value or type, without needing to declare it as mut. This is different from mutability, where the same variable is modified in place. Shadowing is useful for transforming data while keeping immutability in your program.
