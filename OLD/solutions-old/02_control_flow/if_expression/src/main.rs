// Solution: Use a boolean expression for the `if` condition
fn single_condition() {
    let number = 3;

    if number == 3 {
        // Compare `number` to 3 to create a boolean condition
        println!("number was three");
    }
}
// Context: In Rust, the condition in an if statement must be a boolean expression (true or false). Unlike some other languages, you cannot use non-boolean values (like integers) as conditions.

// Solution: Use `if-else` statements to evaluate and print based on conditions
fn multiple_conditions() {
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

// Solution: Ensure all branches of the `if-else` expression return values of the same type
fn inline_expression() {
    let condition = true;

    let number = if condition { 5 } else { 6 }; // Both branches now return integers

    println!("The value of number is: {number}"); // Prints the value of `number`
}
// Context: In Rust, conditional expressions in an if block must return values of the same type for all branches. This ensures type consistency and avoids runtime errors.

fn main() {
    // Execute the function to see if your changes worked!
    single_condition();
    multiple_conditions();
    inline_expression();
}
