// Fix the error in the code by allowing the variable to be mutable
fn variables_2() {
    let x = 1; // `x` is currently immutable
    println!("{x}"); // Prints the initial value of `x`
    x += 1; // This will cause a compilation error. How can you fix it?
    println!("{x}"); // Should print the updated value of `x`
}
