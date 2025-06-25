// Solution: Ensure all branches of the `if-else` expression return values of the same type
fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 }; // Both branches now return integers

    println!("The value of number is: {number}"); // Prints the value of `number`
}
// Context: In Rust, conditional expressions in an if block must return values of the same type for all branches. This ensures type consistency and avoids runtime errors.
