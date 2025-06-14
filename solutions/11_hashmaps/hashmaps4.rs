// Step 0: Import HashMap from the standard library.
use std::collections::HashMap;

fn main() {
    // Step 1: Create a new empty HashMap.
    let mut scores = HashMap::new();

    // Step 2: Add key-value pairs to the HashMap.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Step 3: Retrieve the value associated with the key "Blue".
    let team_name = String::from("Blue");
    // `.get()` returns an Option<&V>, so we use `.copied()` to get an Option<V>,
    // and `.unwrap_or()` to provide a default value if the key doesn't exist.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Step 4: Iterate over the HashMap and print all key-value pairs.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
// Context: The HashMap type in Rust allows you to store key-value pairs, where each key is associated with a specific value. This data structure is particularly useful for fast lookups.
