fn main() {
    // Creating an empty string.
    let mut s = String::new();

    // Converting String Slices.
    let data = "initial contents"; // &str (reference, static, immutable)
    let y = data.to_string(); // String (owned, dynamic, mutable)

    // Use the string function to create a string from the string literal.
    let z = String::from("initial contents");
}
// Context: In Rust, strings come in two primary types: &str (string slices) and String (owned, growable strings). While string slices are static and cannot be modified, the String type provides the flexibility to create and manipulate strings dynamically.
