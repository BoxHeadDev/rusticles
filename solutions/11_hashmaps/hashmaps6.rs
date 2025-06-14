// Solution: overwrite the existing value
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // Insert the initial value.
    scores.insert(String::from("Blue"), 25); // Insert a new value to overwrite the existing value.

    println!("{scores:?}"); // Should print: {"Blue": 25}
}
// Context: In Rust, you can update the value of a key in a HashMap by reinserting the key with a new value. If the key already exists, the new value overwrites the old one.
