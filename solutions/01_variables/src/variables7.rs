// Solution: Add the `mut` keyword to make the variable mutable
fn main() {
    let mut x = 5; // Use `mut` to allow `x` to be modified
    println!("The value of x is: {x}"); // Prints the initial value of `x`
    x = 6; // Now that `x` is mutable, this line will work
    println!("The value of x is: {x}"); // Prints the updated value of `x`
}
// Context: In Rust, variables are immutable by default. This means once you assign a value to a variable, it cannot be changed. To allow modification of a variable's value, you need to explicitly declare it as mutable using the mut keyword.
