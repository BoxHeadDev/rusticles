// Solution: Using `wrapping_add` ensures the value wraps around to 0 instead of causing a panic.
fn main() {
    let max: u8 = 255;
    let total = max.wrapping_add(1);

    println!("Value: {}", total); // This will now print "Value: 0".
}
// Context: Rust’s numeric types, such as u8, have a fixed range of values. For example, a u8 can hold values between 0 and 255. If an operation exceeds this range, it results in an integer overflow. By default, Rust will panic in debug mode when an overflow occurs. However, you can handle this explicitly using methods like wrapping_add, which wraps the value back to the start of the range instead of panicking.
