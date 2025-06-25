#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if the width is greater than 0.
    fn has_width(&self) -> bool {
        self.width > 0
    }

    // Method to update the width of the rectangle.
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Method to return the rectangle with the maximum dimensions.
    fn max(&self, other: &Self) -> Self {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // Method to update the rectangle to have the maximum dimensions.
    fn set_to_max(&mut self, other: &Rectangle) {
        *self = self.max(other);
    }
}
// Context: Structs in Rust can have associated methods implemented via the impl block. These methods allow you to encapsulate functionality related to the struct, making your code more readable and idiomatic. Methods can take &self, &mut self, or even consume self, depending on the desired behaviour.
