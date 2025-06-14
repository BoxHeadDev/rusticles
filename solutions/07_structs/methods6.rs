#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

// Solution:
fn main() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    // Desugared method call for `area`.
    let area1 = Rectangle::area(&r);

    // Desugared method call for `set_width`.
    Rectangle::set_width(&mut r, 2);
}
// Context: In Rust, method calls like r.area() are syntactic sugar for Rectangle::area(&r). Similarly, for methods that require mutable access, r.set_width(2) is shorthand for Rectangle::set_width(&mut r, 2). This desugared form highlights how methods are resolved internally by the compiler.
