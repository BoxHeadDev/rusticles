fn main() {
    // Create an empty vector of integers.
    let mut v1: Vec<i32> = Vec::new();

    // Add integers to the vector.
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    println!("{:?}", v1); // Outputs: [5, 6, 7, 8]
}
// Context: Vectors in Rust are dynamic arrays that can grow or shrink in size. They are one of the most common collection types in Rust, and they require the Vec<T> type to define the type of elements they hold.
