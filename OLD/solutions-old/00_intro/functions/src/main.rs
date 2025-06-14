// Solution: Call `another_function` from within `main`
fn another_function_1() {
    println!("Another function.");
}
// Context: Functions in Rust are an essential way to organize and reuse code. In this exercise, you'll call a function from within the main function.

// Solution: Add the correct type annotations to the parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    // `value` is an integer, and `unit_label` is a character
    println!("The measurement is: {value}{unit_label}"); // Prints the measurement
}
// Context: In Rust, every function parameter must have a type annotation. This ensures that the compiler knows the type of each parameter and can enforce Rust's strict type safety.

// Solution: Use an expression to assign a valid value to `x`
fn expressions() {
    let y = 6; // This is a statement that defines `y`
    let x = y; // Use `y`, which is an expression, to assign a value to `x`
    println!("The value of x is: {x}"); // Prints the value of `x` (6)
}
// Context: In Rust, statements perform actions but do not return values, while expressions evaluate to a value. This distinction is crucial when writing valid Rust code.

// Solution: Add a return type and ensure the function returns a value
fn another_function_2() {
    let x = plus_one(5); // Call `plus_one` with the argument 5
    println!("The value of x is: {x}"); // Prints the returned value (6)
}

fn plus_one(x: i32) -> i32 {
    // Specify that the function returns an `i32`
    x + 1 // Return the result of adding 1 to `x`
}
// Context: In Rust, functions can return values, but the return type must be explicitly declared using ->. If a function does not have a return type, it is assumed to return (), the unit type.

// Solution: The code already works as intended and demonstrates passing block expressions
fn another_function_3() {
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

    another_function_1();

    print_labeled_measurement(5, 'm'); // Should print: "The measurement is: 5m"

    expressions();

    another_function_2();

    another_function_3();
}
