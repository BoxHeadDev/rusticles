#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Solution:
impl Rectangle {
    // Method to determine if this rectangle can hold another rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
// Context: Struct methods in Rust allow you to add behaviour directly to your types. In this challenge, you’ll implement a can_hold method for the Rectangle struct. This method will determine if one rectangle can entirely fit inside another.
