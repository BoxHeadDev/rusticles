struct Color2(i32, i32, i32);

fn main() {
    // Create an instance of the `Color` tuple struct.
    let black = Color2(0, 0, 0); // Represents black in RGB.

    // Destructure the `Color` instance into individual variables.
    let Color2(x, y, z) = black;

    println!("x: {x}, y: {y}, z: {z}");
}
// Context: Tuple structs in Rust can be destructured into individual variables for convenience. This is particularly useful when you need to access or manipulate the individual fields of a tuple struct.
