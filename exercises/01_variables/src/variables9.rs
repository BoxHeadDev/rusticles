// Fix the error by adding a type annotation to the constant
fn main() {
    const THREE_HOURS_IN_SECONDS = 60 * 60 * 3; // This constant is missing a type annotation
    println!(THREE_HOURS_IN_SECONDS); // Should print the value of the constant
}

