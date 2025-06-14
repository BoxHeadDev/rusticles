// Solution: Use a `for` loop with a reversed range
fn main() {
    for number in (1..=4).rev() {
        // Use `1..=4` to include 4 and reverse the range
        println!("{number}!"); // Print each number in the countdown
    }

    println!("LIFTOFF!!!"); // Print "LIFTOFF!!!" after the countdown
}
// Context: In Rust, the for loop can iterate over ranges, and the rev() method can be used to reverse the range for descending iteration.
