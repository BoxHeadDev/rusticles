#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Solution: Associated function to create a square rectangle.
impl Rectangle {
    // Associated function to create a square.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Use the associated function to create a square.
    let sq = Rectangle::square(3);
    println!("{:?}", sq); // Should print: Rectangle { width: 3, height: 3 }
}
// Context: Associated functions, often called "static methods," are functions defined within an impl block but do not operate on an instance of the struct. Instead, they are called directly on the struct itself.
