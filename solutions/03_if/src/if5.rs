// Solution: Use `if-else` statements to evaluate and print based on conditions
fn main() {
    let number = 6;

    if number % 4 == 0 {
        // Check if `number` is divisible by 4
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // Check if `number` is divisible by 3
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // Check if `number` is divisible by 2
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
// Context: In Rust, if-else statements allow you to evaluate multiple conditions and execute the corresponding block of code for the first condition that evaluates to true.
