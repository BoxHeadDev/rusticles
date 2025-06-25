// Solution: Access tuple elements using indices.
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // The first element (500) is at index 0.
    let six_point_four = x.1; // The second element (6.4) is at index 1.
    let one = x.2; // The third element (1) is at index 2.
}
// Context: In Rust, tuples allow you to store multiple values of different types in a single structure. To access individual elements of a tuple, you can use dot notation with an index (e.g., .0, .1, etc.). This is particularly useful when you don't want to destructure the tuple into variables but need specific values.
