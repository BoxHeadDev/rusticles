// Solution: The code works as intended, demonstrating mutable references.
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // Borrow a mutable reference to the third element.
    *num += 1; // Dereference `num` and increment the value it points to.
    println!("Third element is {}", *num); // Print the updated value (should be 4).
    println!("Vector is now {:?}", v); // Print the entire vector (should be [1, 2, 4]).
}
// Context: mutable references allow you to modify values directly through references. The current code uses a mutable reference to update an element in a vector, then safely prints the modified value and the entire vector.
