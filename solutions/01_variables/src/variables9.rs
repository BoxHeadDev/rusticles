// Solution: Add a type annotation to the constant
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Annotate the constant with type `u32`
    println!("{THREE_HOURS_IN_SECONDS}"); // Prints the value of the constant
}
// Context: In Rust, constants are defined using the const keyword, and their types must always be explicitly annotated. Unlike variables, constants are evaluated at compile time and cannot be changed during the program's execution. In this exercise, your task is to add the correct type annotation to a constant to fix the compilation error.
