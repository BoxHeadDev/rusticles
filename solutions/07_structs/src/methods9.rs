#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

// Solution: Use a mutable reference for calling `set_width` through a reference.
fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(1);

    let rect_ref = &mut rect; // Create a mutable reference to `rect`.
    rect_ref.set_width(2); // Now this works because `rect_ref` is mutable.
}
// Context: In Rust, you can create references to variables for borrowing. If a method requires a mutable reference (e.g., &mut self), you must create a mutable reference to the variable. Furthermore, you cannot have both mutable and immutable references to the same variable at the same time.
