// To Do: import HashMap
use std::collections::HashMap;

fn main() {
    hashmap_1();
    hashmap_2();
    hashmap_3();
    hashmap_4();
    hashmap_5();
    hashmap_6();
}

fn hashmap_1() {
    // Step 1: Create new empty HashMap
    let mut scores = HashMap::new();

    // Step 2: Add key value pairs
    // - Blue: 10
    // - Yellow: 50
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Step 3: Assign the value of Blue to score
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Step 4: Loop over scores and print key value
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn hashmap_2() {
    // Will the following compile?
    // No, map takes ownsership of the fields
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{}, {}", field_name, field_value);
}

fn hashmap_3() {
    // Update the value of blue from 10 to 25
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
}

fn hashmap_4() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Step 1: Add Yellow to scores if it doesn't already exist
    // Yellow: 50
    scores.entry(String::from("Yellow")).or_insert(50);

    // Step 2: Update the value of Blue to 50
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

fn hashmap_5() {
    // Add a count for the occurance of each word to map
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn hashmap_6() {
    // Will the following compile?
    // No, h cannot be mutated (h.insert("k2", 1)) while an immutable reference (v1) to it is live.
    let mut h = HashMap::new();
    h.insert("k1", 0);
    let v1 = &h["k1"];
    h.insert("k2", 1);
    let v2 = &h["k2"];
    println!("{} {}", v1, v2);
}
