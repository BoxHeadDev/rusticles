// Solution:
fn main() {
    let s = String::from("hello"); // Create a new `String`, owned by `s`.
    let s2 = add_suffix3(s); // Ownership of `s` is moved to `add_suffix`.
    // `s` is no longer valid here; `s2` now owns the modified string.
    println!("{}", s2); // Prints "hello world".
}

fn add_suffix3(mut s: String) -> String {
    s.push_str(" world"); // Modify the string by appending " world".
    s // Return the modified string, transferring ownership back to the caller.
}
// Context: The function add_suffix takes ownership of a String, modifies it by appending text, and then returns the modified String. The original string (s) is moved to add_suffix, and its ownership is transferred back to the caller as s2.
