// Solution: Add the `mut` keyword and ensure `x` is mutable
fn main() {
    let mut x = 1; // Use `mut` to allow `x` to be modified
    println!("{x}"); // Prints the initial value of `x`
    x += 1; // Increment the value of `x` by 1
    println!("{x}"); // Prints the updated value of `x`
}
// Context: In Rust, variables are immutable by default. This means once you assign a value to a variable, it cannot be changed. To allow modification of a variable's value, you need to explicitly declare it as mutable using the mut keyword.
