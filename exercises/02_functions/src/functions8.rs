// Fix the code so it compiles and correctly demonstrates statements and expressions
fn expressions() {
    let x = (let y = 6); // `let y = 6` is a statement, not an expression
    println!("The value of x is: {x}"); // This line should print a valid value for `x`
}


fn main() {
    // Execute the function to see if your changes worked!

    expressions();
}
