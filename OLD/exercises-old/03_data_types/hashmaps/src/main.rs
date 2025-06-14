// Step 0: Import HashMap from the standard library.

fn hashmap_1() {
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

// Fix the following to make the program compile.
fn hashmap_2() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{}, {}", field_name, field_value);
}

// Update the value of blue from 10 to 25.
fn hashmap_3() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // Hint: Insert a new value for the key "Blue" to overwrite the existing value.

    println!("{scores:?}"); // Should print: {"Blue": 25}
}

// Update scores so it prints {"Blue": 10, "Yellow": 50}
fn hashmap_4() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Step 1: Add Yellow to scores if it doesn't already exist
    // Hint: Use the `.entry()` API with `.or_insert()` to add "Yellow" with a value of 50.

    // Step 2: Update the value of Blue to 50
    // Hint: Use the `.entry()` API with `.or_insert()` to ensure Blue's value is updated.

    println!("{scores:?}"); // Should print: {"Blue": 10, "Yellow": 50}
}

// Add a count for the occurrence of each word to the map
fn hashmap_5() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Hint: Use a for loop and `text.split_whitespace()` to iterate over the words.
    // Use `.entry()` and `.or_insert()` to update the count for each word.

    println!("{map:?}"); // Should print: {"hello": 1, "world": 2, "wonderful": 1}
}

// Will the following compile? Why or why not?
fn hashmap_6() {
    let mut h = HashMap::new();
    h.insert("k1", 0);

    let v1 = &h["k1"]; // Immutable reference to h["k1"]
    
    // Hint: This causes an issue because h is being mutated while v1 is still in use.
    h.insert("k2", 1); // Mutable borrow of h occurs here.

    let v2 = &h["k2"]; // Immutable reference to h["k2"]
    println!("{} {}", v1, v2);
}

fn main() {
    // Execute the function to see if your changes worked!
    hashmap_1();
    hashmap_2();
    hashmap_3();
    hashmap_4();
    hashmap_5();
    hashmap_6();
}
