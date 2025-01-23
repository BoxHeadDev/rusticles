// Fix the condition in the `if` statement to make it a valid boolean expression
fn single_condition() {
    let number = 3;

    if number {
        // This condition is invalid in Rust because `number` is not a boolean
        println!("number was three");
    }
}

// Fix the code to use `if-else` statements for evaluating divisibility
fn multiple_conditions() {
    let number = 6;

    // Check if `number` is divisible by 4
    println!("number is divisible by 4");

    // Check if `number` is divisible by 3
    println!("number is divisible by 3");

    // Check if `number` is divisible by 2
    println!("number is divisible by 2");

    println!("number is not divisible by 4, 3, or 2");
}

// Fix the code so that the conditional expression returns values of the same type
fn inline_expression() {
    let condition = true;

    let number = if condition { 5 } else { "six" }; // The branches return different types (integer and string)

    println!("The value of number is: {number}");
}

fn main() {
    // Execute the function to see if your changes worked!
    single_condition();
    multiple_conditions();
    inline_expression();
}
