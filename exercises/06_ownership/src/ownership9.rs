// Fix the code to handle ownership transfer properly.
// The goal is to ensure values are used safely and ownership is transferred without errors.
fn main() {
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b; // Ownership of the Box is moved to `b2`.
    println!("{}", b); // ERROR: `b` is no longer valid after ownership is moved to `b2`.
    move_a_box(b2); // Transfer ownership of `b2` to the function.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}
