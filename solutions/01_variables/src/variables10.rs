// Solution: Remove mutability and use separate constants for different values
fn main() {
    const TOTAL: u32 = 50; // Declare a constant with an explicit type annotation
    println!("{TOTAL}"); // Prints the value of TOTAL

    const NEW_TOTAL: u32 = 100; // Use a new constant for the updated value
    println!("{NEW_TOTAL}"); // Prints the value of NEW_TOTAL
}
// Context: In Rust, constants declared with the const keyword are always immutable. Unlike variables marked with let, constants cannot be changed after they are declared. Additionally, constants do not support the mut keyword, and attempting to modify them will result in a compilation error.
