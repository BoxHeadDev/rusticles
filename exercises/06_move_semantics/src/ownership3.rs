// Modify the code to demonstrate the ownership transfer between `a` and `b`.
// The goal is to show what happens when ownership of a `Box` is moved.
fn main() {
    let a = Box::new([0; 1_000_000]); // A heap-allocated array.
    let b = a; // Ownership of the array is moved from `a` to `b`.
    // Attempting to use `a` after this point will result in an error.
}
