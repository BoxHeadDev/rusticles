// Fix the code to avoid using a variable (`b`) after its ownership has been moved.
// The goal is to ensure ownership is handled safely without invalid references.
fn main() {
    let b = Box::new(0); // Create a new Box containing 0.
    move_a_box(b); // Transfer ownership of `b` to the function.
    println!("{}", b); // ERROR: `b` is no longer valid after its ownership was moved.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}
