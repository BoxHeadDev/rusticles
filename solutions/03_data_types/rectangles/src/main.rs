#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the rectangle for debugging purposes.
    println!("rect1 is {rect1:#?}");

    // Pass the rectangle to the area function.
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    // Use the struct fields to calculate the area.
    rectangle.width * rectangle.height
}
// Context: In Rust, grouping related values into a struct makes the code more organised, readable, and easier to manage. Instead of passing multiple related variables to a function, you can pass a single instance of a struct. This refactoring improves the clarity and maintainability of your code.
