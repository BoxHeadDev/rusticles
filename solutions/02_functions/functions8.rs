// Solution: Use an expression to assign a valid value to `x`
fn expressions() {
    let y = 6; // This is a statement that defines `y`
    let x = y; // Use `y`, which is an expression, to assign a value to `x`
    println!("The value of x is: {x}"); // Prints the value of `x` (6)
}
// Context: In Rust, statements perform actions but do not return values, while expressions evaluate to a value. This distinction is crucial when writing valid Rust code.

fn main() {
    // Execute the function to see if your changes worked!

    expressions();
}
