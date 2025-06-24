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

// Will the following compile? Why or why not?
fn main() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    println!("{}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(other_rect);
    // Will `rect` still be usable here? Why or why not?
}
