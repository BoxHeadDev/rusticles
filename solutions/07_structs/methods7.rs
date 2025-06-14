#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn max(&self, other: &Self) -> Self {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

// Solution:
fn main() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    println!("{}", rect.area()); // This works because `area` borrows `rect`.

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(&other_rect); // `rect` and `other_rect` are consumed here.

    // `rect` is no longer usable here because it was moved into the `max` method.
    // Uncommenting the line below will cause a compile error.
    // println!("{}", rect.area());
}
// Context: In Rust, methods can borrow or take ownership of a struct instance. When a method takes ownership (e.g., self instead of &self), the instance is consumed and cannot be used afterward. If a method borrows the instance (e.g., &self or &mut self), it allows the instance to remain accessible after the method call.
