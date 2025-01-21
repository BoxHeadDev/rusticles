fn main() {
    // Will the following compile?
    // What is the output?
    single_condition();
    multiple_conditions();
    inline_expression();
}

fn single_condition() {
    let number = 3;

    if number == 3 {
        println!("number was three");
    }
}

fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Condition needs to return same type
fn inline_expression() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
