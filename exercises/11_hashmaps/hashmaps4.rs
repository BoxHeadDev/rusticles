// Step 0: Import HashMap from the standard library.

fn main() {
    // Step 1: Create a new empty HashMap.
    let mut scores = ??;

    // Step 2: Add key-value pairs to the HashMap.
    // Hint: Use the `.insert()` method to add the following pairs:
    // - "Blue": 10
    // - "Yellow": 50

    // Step 3: Retrieve the value associated with the key "Blue".
    let team_name = String::from("Blue");
    let score = ??; // Hint: Use `.get()` to retrieve the value and handle the Option.

    // Step 4: Iterate over the HashMap and print all key-value pairs.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
