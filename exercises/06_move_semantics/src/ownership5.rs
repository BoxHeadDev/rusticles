// Fix the code to ensure proper cloning and ownership handling.
// The goal is to create a clone of `second` and use it safely.
fn main() {
    let second = String::from("Ferris"); // Create a new `String`.
    let second_clone = second.clone(); // Clone `second` to preserve its original value.
    let whole = add_suffix(first_clone); // ERROR: `first_clone` is not defined; use `second_clone`.
    println!("{whole}, originally {second}"); // Print both the modified and original strings.
}

fn add_suffix(mut s: String) -> String {
    s.push_str(" Rustacean"); // Append a suffix to the string.
    s // Return the modified string.
}
