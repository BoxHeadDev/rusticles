// Solution: The code is fixed to use `second_clone` correctly.
fn main() {
    let second = String::from("Ferris"); // Create a new `String`.
    let second_clone = second.clone(); // Clone `second` to preserve the original value.
    let whole = add_suffix2(second_clone); // Pass the cloned value to `add_suffix`.
    println!("{whole}, originally {second}"); // Safely print both the modified and original strings.
}

fn add_suffix2(mut s: String) -> String {
    s.push_str(" Rustacean"); // Append a suffix to the string.
    s // Return the modified string.
}
// Context: The original code uses .clone() to create a duplicate of a String value (second) so that the cloned version can be passed to a function (add_suffix) without affecting the original string. This ensures the original string remains valid and accessible after the function call.
