fn main() {
    // Will the following compile?
    // What is the output?
    single_condition();
    multiple_conditions();
    inline_expression();
}

fn single_condition() {
    let number = 3;

    if number {
        println!("number was three");
    }
}

fn multiple_conditions() {
    let number = 6;

    println!("number is divisible by 4");
    println!("number is divisible by 3");
    println!("number is divisible by 2");
    println!("number is not divisible by 4, 3, or 2");
}

fn inline_expression() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
