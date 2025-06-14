// Solution: The code already works as intended and demonstrates passing block expressions
fn another_function() {
    println!(
        "{}",
        f({
            let y = 1; // Declare and initialize `y`
            y + 1 // This block evaluates to `2` and is passed to `f`
        })
    ); // Output: 3
}

fn f(x: i32) -> i32 {
    x + 1 // Add 1 to the input value and return it
}
// Context: In Rust, blocks enclosed in {} are expressions that can return values. You can pass such expressions as arguments to functions. This exercise demonstrates how to use a block as an expression and pass its value to a function.

fn main() {
    // Execute the function to see if your changes worked!

    another_function();
}
