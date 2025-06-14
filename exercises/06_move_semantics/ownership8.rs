// Fix the code to handle ownership transfer properly.
// The goal is to ensure `Box` ownership is transferred safely without ambiguity.
fn main() {
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b; // Ownership of the Box is moved to `b2`.
    move_a_box(b); // ERROR: Ownership of `b` has already been moved, so this is invalid.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}
