// To Do: import HashMap

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
    let mut scores = ??;

    // Step 2: Add key value pairs
    // - Blue: 10
    // - Yellow: 50

    // Step 3: Assign the value of Blue to score
    let team_name = String::from("Blue");
    let score = ??;

    // Step 4: Loop over scores and print key value
    println!("{key}: {value}");
}

fn hashmap_2() {
    // Will the following compile?
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

    println!("{scores:?}");
}

fn hashmap_4() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    // Step 1: Add Yellow to scores if it doesn't already exist
    // Yellow: 50

    // Step 2: Update the value of Blue to 50

    println!("{scores:?}");
}

fn hashmap_5() {
    // Add a count for the occurance of each word to map
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    println!("{map:?}");
}

fn hashmap_6() {
    // Will the following compile?
    let mut h = HashMap::new();
    h.insert("k1", 0);
    let v1 = &h["k1"];
    h.insert("k2", 1);
    let v2 = &h["k2"];
    println!("{} {}", v1, v2);
}
