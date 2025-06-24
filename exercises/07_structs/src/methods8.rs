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
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(0); // This method requires mutable access to `rect`.
}
