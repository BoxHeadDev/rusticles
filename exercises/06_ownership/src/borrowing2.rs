// Fix this code to make it compile and behave as intended!
// The goal is to properly dereference and borrow the values stored in a `Box`.
fn main() {
    let mut x: Box<i32> = Box::new(1);

    let a: i32 = x; // ERROR: Cannot move value out of a `Box`.
    x += 1; // ERROR: Need to modify the value inside the `Box`.

    let r1: &Box<i32> = x; // ERROR: Cannot directly borrow `x` like this.
    let b: i32 = r1; // ERROR: Dereferencing is required here.

    let r2: &i32 = x; // ERROR: Need to borrow the value stored inside the `Box`.
    let c: i32 = r2; // ERROR: Missing dereference to access the value.
}
