// Complete the code to ensure it works correctly with mutable references.
// The goal is to modify the third element of the vector using a mutable reference.
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // Borrow a mutable reference to the third element.
    *num += 1; // Modify the value through the mutable reference.
    println!("Third element is {}", *num); // Print the updated value.
    println!("Vector is now {:?}", v); // Print the entire vector.
}
