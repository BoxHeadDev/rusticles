// Define two tuple structs: `Color` and `Point`.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // Create an instance of the `Color` tuple struct.
    let black = Color(0, 0, 0); // Represents black in RGB.

    // Create an instance of the `Point` tuple struct.
    let origin = Point(0, 0, 0); // Represents the origin in 3D space.
}
// Context: Tuple structs in Rust are a hybrid between tuples and traditional structs. They group related values together but use positional fields instead of named ones. Tuple structs are useful when you want a lightweight structure with less verbosity than named structs.
