// Fix the code to avoid using `first` after its ownership is moved to `add_suffix`.
// The goal is to safely handle ownership while preserving access to the original string value.
fn main() {
    let first = String::from("Ferris"); // Create a new `String`.
    let full = add_suffix(first); // Ownership of `first` is moved here.
    println!("{full}, originally {first}"); // ERROR: `first` is no longer valid.
}

fn add_suffix(mut s: String) -> String {
    s.push_str(" world"); // Append a suffix to the string.
    s // Return the modified string.
}
