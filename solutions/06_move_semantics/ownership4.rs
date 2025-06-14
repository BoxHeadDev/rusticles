// Solution: The code is fixed by cloning `first` before passing ownership or reordering operations.
fn main() {
    let first = String::from("Ferris"); // Create a new `String`.
    let full = add_suffix(first.clone()); // Clone `first` to preserve its value.
    println!("{full}, originally {first}"); // Safely print both the original and modified strings.
}

fn add_suffix(mut s: String) -> String {
    s.push_str(" world"); // Append a suffix to the string.
    s // Return the modified string.
}
// Context: The original code attempts to use the variable first after its ownership has been moved to the add_suffix function. Rust prevents this because once ownership is transferred, the original variable is no longer valid.
