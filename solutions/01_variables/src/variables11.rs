// Solution: Declare the constant with global scope
const THREE: u32 = 1 + 2; // A global constant that is accessible from anywhere in the program

fn main() {
    println!("{THREE}"); // Prints the value of the constant (3)
}
// Context: In Rust, constants declared with the const keyword can have a global scope, making them accessible from any part of your program. Unlike variables declared inside functions, constants are evaluated at compile time and remain immutable throughout the program's execution.
