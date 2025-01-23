// Modify the `main` function to call `another_function` 
fn another_function_1() {
    println!("Another function.");
}

// Fix the function by adding the correct type annotations to the parameters
fn print_labeled_measurement(value, unit_label) {
    println!("The measurement is: {value}{unit_label}");
}

// Fix the code so it compiles and correctly demonstrates statements and expressions
fn expressions() {
    let x = (let y = 6); // `let y = 6` is a statement, not an expression
    println!("The value of x is: {x}"); // This line should print a valid value for `x`
}

// Fix the `plus_one` function to include a return type and return the correct value
fn another_function_2() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) {
    x + 1; // This line doesn't currently return a value
}

// Fix the code to correctly pass a block expression as an argument to the function `f`
fn another_function_3() {
    println!(
        "{}",
        f({
            let y = 1;
            y + 1 // This block evaluates to a value
        }) // Pass the result of this block to the function `f`
    ); // What should this print?
}

fn f(x: i32) -> i32 {
    x + 1 // Add 1 to the input value and return it
}

fn main() {
    // Execute the function to see if your changes worked!

    print_labeled_measurement(); // Should print: "The measurement is: 5m"

    expressions();
    
    another_function_2();

    another_function_3();
}
