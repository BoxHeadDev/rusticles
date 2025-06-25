// Solution: Implement a `while` loop to count down from 3 to 1
fn main() {
    let mut number = 3;

    while number != 0 {
        // Continue looping as long as `number` is not 0
        println!("{number}!"); // Print the current value of `number`

        number -= 1; // Decrement `number` by 1 on each iteration
    }

    println!("LIFTOFF!!!"); // Print "LIFTOFF!!!" after the countdown is complete
}
// Context: In Rust, a while loop allows you to repeatedly execute a block of code as long as a specified condition evaluates to true.
