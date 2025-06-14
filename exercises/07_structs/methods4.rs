#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement the `can_hold` method for the Rectangle struct.
fn can_hold(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    rect1.width > rect2.width && rect1.height > rect2.height
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

    // Use the `can_hold` method to determine if rect1 can hold rect2 and rect3.
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
