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

// Fix the following code so it compiles.
fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 0,
    };
    rect.set_to_max(other_rect); // Fix the method conflict.
}
