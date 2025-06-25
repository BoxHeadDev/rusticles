// Solution: The code is fixed by separating the mutable and immutable borrows.
fn main() {
    let mut a = [0, 1, 2, 3]; // `a` needs to be mutable to modify its contents
    let y = a[2]; // First, take a copy of the value at index 2
    let x = &mut a[1]; // Now, safely create a mutable reference to index 1
    *x += y; // Modify the value at index 1
    // Both borrows have ended here, and further operations on `a` are safe.
    println!("{:?}", a); // Output the modified array.
}
// Context: The original code attempts to create both a mutable reference (x) to one element of an array and an immutable reference (y) to another element. While these references target different indices, Rust's compiler doesn't analyse the indices and assumes potential aliasing, leading to a borrow conflict.
