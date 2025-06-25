// Solution: Add a return type and ensure the function returns a value
fn another_function() {
    let x = plus_one(5); // Call `plus_one` with the argument 5
    println!("The value of x is: {x}"); // Prints the returned value (6)
}

fn plus_one(x: i32) -> i32 {
    // Specify that the function returns an `i32`
    x + 1 // Return the result of adding 1 to `x`
}
// Context: In Rust, functions can return values, but the return type must be explicitly declared using ->. If a function does not have a return type, it is assumed to return (), the unit type.

fn main() {
    // Execute the function to see if your changes worked!

    another_function();
}
