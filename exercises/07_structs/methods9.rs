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

// Fix the following code so it compiles.
fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(1);

    let rect_ref = &rect; // Hint: This should be a mutable reference to `rect`.
    rect_ref.set_width(2); // This method requires a mutable reference.
}
