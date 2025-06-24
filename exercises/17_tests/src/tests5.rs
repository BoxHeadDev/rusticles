// 🦀 Rustlings Challenge: Checking Results with assert!
//
// Your task is to:
// ✅ Write two tests:
//    - One where a larger rectangle *can* hold a smaller one (should pass)
//    - One where a smaller rectangle *cannot* hold a larger one (should pass)
//
// You'll use `assert!(...)` to validate the method's return value.
//
// HINT: A rectangle can hold another if its width > other's width AND its height > other's height.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // TODO: Create a larger and smaller rectangle
        // Assert that larger.can_hold(&smaller) is true
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        // TODO: Same rectangles as above
        // Assert that smaller.can_hold(&larger) is false using `assert!(!...)`
    }
}
