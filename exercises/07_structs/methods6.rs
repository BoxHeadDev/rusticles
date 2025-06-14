#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

// Rewrite the method calls in `struct_5` using their equivalent function syntax.
fn main() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    let area1 = r.area(); // Rewrite this using the function syntax.
    r.set_width(2); // Rewrite this using the function syntax.
}
