#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn has_width(rectangle: &Rectangle) -> bool {
    rectangle.width > 0
}

// Refactor this to use a method on the `Rectangle` struct.
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if has_width(&rect1) {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
