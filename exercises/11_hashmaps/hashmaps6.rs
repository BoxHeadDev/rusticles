// Update the value of blue from 10 to 25.
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // Hint: Insert a new value for the key "Blue" to overwrite the existing value.

    println!("{scores:?}"); // Should print: {"Blue": 25}
}
