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
// Updated max method to take an immutable reference to `other`.
fn main() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    // Pass an immutable reference to `other_rect` to the `max` method.
    let max_rect = rect.max(&other_rect);
    println!("{}", rect.area()); // Works because `rect` is borrowed immutably.
}
// Context: In Rust, methods can take ownership of their parameters, borrow them immutably (&), or borrow them mutably (&mut). By using references, you can avoid consuming variables, allowing them to remain usable after the method call.
