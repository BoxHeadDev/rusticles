// Fix the code so that the conditional expression returns values of the same type
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" }; // The branches return different types (integer and string)

    println!("The value of number is: {number}");
}
