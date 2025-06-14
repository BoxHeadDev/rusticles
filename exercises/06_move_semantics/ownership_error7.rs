// Fix the code to resolve conflicting borrows in the array.
// The goal is to safely access and modify different elements of the array.
fn main() {
    let a = [0, 1, 2, 3];
    let x = &mut a[1]; // Mutable borrow of the second element.
    let y = &a[2]; // ERROR: Immutable borrow of the third element conflicts with the mutable borrow.
    *x += *y; // Attempt to modify the second element using the immutable third element.
}
