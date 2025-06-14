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

// Solution: Mark `rect` as mutable to allow calling the `set_width` method.
fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(0); // Now this works because `rect` is mutable.
}
// Context: In Rust, methods that modify a struct instance require mutable access. If the struct instance is not declared as mut, you’ll encounter a compile-time error. This ensures that modifications to a struct instance are explicit and safe.
