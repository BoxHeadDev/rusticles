fn main() {
    // Define an enum to handle different types of data in the vector.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Create a vector of `SpreadsheetCell` to store heterogeneous data.
    let row = vec![
        SpreadsheetCell::Int(3),                     // Store an integer.
        SpreadsheetCell::Text(String::from("blue")), // Store a string.
        SpreadsheetCell::Float(10.12),               // Store a float.
    ];
}
// Context: Rust vectors can only hold elements of a single type. If you need to store heterogeneous data (e.g., integers, floats, and strings) in the same vector, you can define an enum with variants for each type. By wrapping the values in the enum, you can use the vector to store different types in a type-safe manner.
