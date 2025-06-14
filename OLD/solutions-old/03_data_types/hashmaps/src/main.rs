// Step 0: Import HashMap from the standard library.
use std::collections::HashMap;

fn hashmap_1() {
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

// Solution:
fn hashmap_2() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // The `String`s are moved into the `HashMap`, so their ownership is transferred.
    map.insert(&field_name, &field_value); // Reference or clone the strings to retain ownership.

    // Since we referenced the values before inserting, the original variables are still usable.
    println!("{}, {}", field_name, field_value);
}
// Context: In Rust, when inserting values into a HashMap, the HashMap takes ownership of the keys and values. After the insertion, you can no longer use the original variables unless they implement the Copy trait (like integers) or are borrowed. This behaviour prevents data races and ensures memory safety.

// Solution: overwrite the existing value
fn hashmap_3() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // Insert the initial value.
    scores.insert(String::from("Blue"), 25); // Insert a new value to overwrite the existing value.

    println!("{scores:?}"); // Should print: {"Blue": 25}
}
// Context: In Rust, you can update the value of a key in a HashMap by reinserting the key with a new value. If the key already exists, the new value overwrites the old one.

// Solution:
fn hashmap_4() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Step 1: Add Yellow to scores if it doesn't already exist
    scores.entry(String::from("Yellow")).or_insert(50);

    // Step 2: Update the value of Blue to 50
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}"); // Should print: {"Blue": 10, "Yellow": 50}
}
// Context: The entry API in Rust's HashMap provides a powerful way to add or update values conditionally. Using entry, you can insert a value for a key only if the key does not already exist. You can also update the value of an existing key using methods like .or_insert().

// Solution:
fn hashmap_5() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Split the text into words and count each occurrence.
    for word in text.split_whitespace() {
        // Get a mutable reference to the count for the word or insert 0 if it doesn't exist.
        let count = map.entry(word).or_insert(0);
        *count += 1; // Increment the count.
    }

    println!("{map:?}"); // Should print: {"hello": 1, "world": 2, "wonderful": 1}
}
// Context: A HashMap is an excellent tool for counting the occurrences of items in a collection. In this challenge, you’ll use a HashMap to count how many times each word appears in a given text. The .entry() API with .or_insert() will be your primary tool for incrementing the count for each word.

// This code won't compile as is because Rust prevents mutable and immutable borrows simultaneously.
fn hashmap_6() {
    let mut h = HashMap::new();
    h.insert("k1", 0);

    // To fix this, we drop the immutable reference (v1) before mutating h.
    let v1 = h["k1"]; // Take a copy of the value instead of an immutable reference.

    h.insert("k2", 1); // Now we can safely mutate h.

    let v2 = h["k2"]; // Take another copy of the value.
    println!("{} {}", v1, v2); // Prints: "0 1"
}
// Context: In Rust, borrowing rules ensure memory safety by preventing mutable and immutable references to the same data at the same time. When working with a HashMap, you cannot modify it while an immutable reference to one of its elements is still active.

fn main() {
    // Execute the function to see if your changes worked!
    hashmap_1();
    hashmap_2();
    hashmap_3();
    hashmap_4();
    hashmap_5();
    hashmap_6();
}
