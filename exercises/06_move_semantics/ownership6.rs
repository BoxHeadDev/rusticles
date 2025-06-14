// Modify the code to ensure proper ownership transfer and understand how Rust handles returning values.
fn main() {
    let s = String::from("hello"); // Create a new `String`.
    let s2 = add_suffix(s); // Transfer ownership of `s` to `add_suffix`.
    println!("{}", s2); // Print the modified string.
}

fn add_suffix(mut s: String) -> String {
    s.push_str(" world"); // Append " world" to the string.
    s // Return the modified string, transferring ownership back to the caller.
}
