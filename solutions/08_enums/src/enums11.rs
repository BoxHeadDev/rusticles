// Solution: extract the value from the `Option<i8>` type.
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // `y` is an `Option<i8>`.

    // Use `.unwrap()` to extract the value from the `Option` for this example.
    let y_value = y.unwrap(); // This will panic if `y` is `None`.

    // Bonus: Provide a default value if `y` is `None`.
    // let y_value = y.unwrap_or(0);

    let sum = x + y_value; // Now you can safely add the two `i8` values.
    println!("The sum is: {}", sum); // Output: The sum is: 10
}
// Context: The Option enum is used to represent a value that may or may not exist, but it cannot be directly used in arithmetic operations without first handling the Option type. In Rust, you need to extract the value inside the Option using pattern matching or helper methods like .unwrap() (for learning purposes) to convert the Option into its inner type.
