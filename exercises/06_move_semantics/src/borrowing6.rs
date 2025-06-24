// Fix the code to handle a mutable reference and a downgraded immutable reference.
// Ensure the code compiles and runs correctly without violating borrowing rules.
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // Mutable reference to the third element.
    let num2: &i32 = &*num; // ERROR: Properly downgrade the mutable reference to an immutable one.
    println!("{} {}", *num, *num2); // Print both references.
}
