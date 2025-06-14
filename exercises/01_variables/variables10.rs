// Fix the code so that it compiles and follows Rust's rules for constants
fn main() {
    const mut TOTAL = 50; // Constants cannot be mutable
    println!(TOTAL); // Prints the initial value of TOTAL
    TOTAL = 100; // This will cause a compilation error. What can you do instead?
    println!(TOTAL); // Prints the new value of TOTAL
}
