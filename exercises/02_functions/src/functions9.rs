// Fix the `plus_one` function to include a return type and return the correct value
fn another_function() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) {
    x + 1; // This line doesn't currently return a value
}

fn main() {
    another_function();
}
