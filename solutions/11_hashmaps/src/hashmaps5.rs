// Solution:
fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // The `String`s are moved into the `HashMap`, so their ownership is transferred.
    map.insert(&field_name, &field_value); // Reference or clone the strings to retain ownership.

    // Since we referenced the values before inserting, the original variables are still usable.
    println!("{}, {}", field_name, field_value);
}
// Context: In Rust, when inserting values into a HashMap, the HashMap takes ownership of the keys and values. After the insertion, you can no longer use the original variables unless they implement the Copy trait (like integers) or are borrowed. This behaviour prevents data races and ensures memory safety.
