// Solution:
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Step 1: Add Yellow to scores if it doesn't already exist
    scores.entry(String::from("Yellow")).or_insert(50);

    // Step 2: Update the value of Blue to 50
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}"); // Should print: {"Blue": 10, "Yellow": 50}
}
// Context: The entry API in Rust's HashMap provides a powerful way to add or update values conditionally. Using entry, you can insert a value for a key only if the key does not already exist. You can also update the value of an existing key using methods like .or_insert().
