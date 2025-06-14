#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn set_to_max(&mut self, other: &Rectangle) {
        *self = self.max(other);
    }
}

// Solution:
// Update `max` and `set_to_max` to borrow `other` instead of taking ownership.
fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 0,
    };
    rect.set_to_max(&other_rect); // Pass a reference to `other_rect` instead of moving it.
    println!("{:?}", rect); // Outputs: Rectangle { width: 1, height: 1 }
}
// Context: In Rust, methods that borrow (&self) or mutably borrow (&mut self) can conflict with methods that take ownership (self) of a parameter. When chaining or combining methods, these conflicts can lead to compile-time errors. This is because Rust enforces strict borrowing rules to ensure memory safety.
