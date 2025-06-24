// Update scores so it prints {"Blue": 10, "Yellow": 50}
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Step 1: Add Yellow to scores if it doesn't already exist
    // Hint: Use the `.entry()` API with `.or_insert()` to add "Yellow" with a value of 50.

    // Step 2: Update the value of Blue to 50
    // Hint: Use the `.entry()` API with `.or_insert()` to ensure Blue's value is updated.

    println!("{scores:?}"); // Should print: {"Blue": 10, "Yellow": 50}
}
