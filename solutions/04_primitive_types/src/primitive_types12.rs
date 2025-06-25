// Solution: Add explicit type annotations to a tuple and destructure the tuple into separate variables to print each value.
fn main() {
    // Adding explicit type annotations makes the tuple's structure clear.
    let tup: (i32, f32, u8) = (500, 6.4, 1);

    // Destructuring the tuple into individual variables.
    let (x, y, z) = tup;

    println!("The value of x is: {x}"); // Prints: The value of x is: 500
    println!("The value of y is: {y}"); // Prints: The value of y is: 6.4
    println!("The value of z is: {z}"); // Prints: The value of z is: 1
}
// Context: Tuples in Rust can hold multiple values of different types. By default, Rust can infer the types of tuple elements, but adding explicit type annotations improves code clarity. Tuples can also be destructured into individual variables for convenient access to their elements.
