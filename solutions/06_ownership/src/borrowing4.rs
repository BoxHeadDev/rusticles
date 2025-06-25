// Solution: This version fixes the borrowing and mutation issue by rearranging the operations.
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4); // Mutate the vector before borrowing any reference.
    let num: &i32 = &v[2]; // Borrow an immutable reference after mutation is complete.
    println!("Third element is {}", *num); // Now `num` is valid and safe to use.
}
// Context: The code attempted to borrow an immutable reference to an element in a vector while also mutating the vector. This is not allowed because mutation can invalidate references, potentially causing undefined behaviour.
