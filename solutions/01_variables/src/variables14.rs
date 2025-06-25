// Solution: The code is correct as-is, demonstrating shadowing and scope isolation
fn main() {
    let mut x: u32 = 1; // Outer mutable variable `x`
    {
        let mut x = x; // Shadow the outer `x` with a new mutable variable
        x += 2; // Modify the inner `x` (this does not affect the outer `x`)
    }
    println!("{x}"); // Prints 1 because the outer `x` remains unchanged
}
// Context: In Rust, shadowing allows you to create a new variable with the same name as an existing one, even inside inner scopes. However, shadowing in an inner scope does not affect the value of the outer variable. This exercise helps you understand how shadowing works with scope isolation and demonstrates that changes made in an inner scope do not "leak" to the outer scope.
