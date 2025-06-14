#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to check if the width is greater than 0.
    fn has_width(&self) -> bool {
        self.width > 0
    }
}

// Use the `width` method instead of a standalone function.
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.has_width() {
        println!("The rectangle has a width; it is {}", rect1.width);
    }
}
// Context: Structs can have methods that encapsulate functionality related to their fields. In Rust, using methods instead of accessing fields directly or relying on standalone functions makes the code more concise, modular, and intuitive.
