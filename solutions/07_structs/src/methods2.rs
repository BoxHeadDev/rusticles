#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Use the `area` method on the `Rectangle` struct.
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
// Context: When working with structs, it's more idiomatic in Rust to define methods within an impl block rather than relying on standalone functions. Methods allow you to directly operate on an instance of the struct, making your code cleaner and more intuitive.
