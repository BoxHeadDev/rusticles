// Fix the condition in the `if` statement to make it a valid boolean expression
fn main() {
    let number = 3;

    if number {
        // This condition is invalid in Rust because `number` is not a boolean
        println!("number was three");
    }
}
