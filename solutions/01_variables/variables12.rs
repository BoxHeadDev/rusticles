// Solution: The code demonstrates shadowing and scopes effectively
fn main() {
    let x = 5; // Initial value of `x`

    let x = x + 1; // Shadow `x` with a new value (5 + 1 = 6)

    {
        let x = x * 2; // Shadow `x` again in the inner scope (6 * 2 = 12)
        println!("The value of x in the inner scope is: {x}"); // Prints 12
    }

    println!("The value of x is: {x}"); // Prints 6 (the outer `x` is unaffected by the inner scope)
}
// Context: Rust allows shadowing, where you can declare a new variable with the same name as a previous variable. This feature allows you to reuse variable names without mutability, enabling transformations of values while keeping them immutable. Shadowing also respects scope boundaries, meaning changes in an inner scope do not affect the outer scope.
