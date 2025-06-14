// Solution: Add the correct type annotations to the parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    // `value` is an integer, and `unit_label` is a character
    println!("The measurement is: {value}{unit_label}"); // Prints the measurement
}
// Context: In Rust, every function parameter must have a type annotation. This ensures that the compiler knows the type of each parameter and can enforce Rust's strict type safety.

fn main() {
    // Execute the function to see if your changes worked!

    print_labeled_measurement(5, 'm'); // Should print: "The measurement is: 5m"
}
