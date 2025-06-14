// Fix the integer overflow by using a method to handle overflow explicitly.
fn main() {
    let max: u8 = 255; // Hint: 255 is the maximum value for the u8 type.
    let total = max + 1; // This causes an integer overflow.

    println!("Value: {}", total);
}
