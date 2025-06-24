#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Refactor the following function into an associated function within the `Rectangle` struct.
fn main() {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    let sq = square(3); // Use the refactored associated function here.
    println!("{:?}", sq); // Should print: Rectangle { width: 3, height: 3 }
}
