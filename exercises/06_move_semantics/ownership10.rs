// Fix the code to handle ownership transfer properly.
// The goal is to ensure `Box` ownership is safely managed without using invalid variables.
fn main() {
    let b = Box::new(0); // Create a new Box containing 0.
    move_a_box(b); // Transfer ownership of `b` to the function.
    let b2 = b; // ERROR: `b` is no longer valid after its ownership was moved.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}
