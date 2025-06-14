// Fix the code to correctly pass a block expression as an argument to the function `f`
fn another_function() {
    println!(
        "{}",
        f({
            let y = 1;
            y + 1 // This block evaluates to a value
        }) // Pass the result of this block to the function `f`
    ); // What should this print?
}

fn f(x: i32) -> i32 {
    x + 1 // Add 1 to the input value and return it
}

fn main() {
    // Execute the function to see if your changes worked!

    another_function();
}
