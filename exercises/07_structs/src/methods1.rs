#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Refactor the following standalone functions into methods on the Rectangle struct.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn has_width(rectangle: &Rectangle) -> bool {
    rectangle.width > 0
}

fn set_width(rectangle: &mut Rectangle, width: u32) {
    rectangle.width = width;
}

fn max(rectangle: Rectangle, other: Rectangle) -> Rectangle {
    Rectangle {
        width: rectangle.width.max(other.width),
        height: rectangle.height.max(other.height),
    }
}

fn set_to_max(rectangle: &mut Rectangle, other: Rectangle) {
    *rectangle = max(*rectangle, other);
}
