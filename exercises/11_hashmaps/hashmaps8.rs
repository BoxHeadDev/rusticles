// Add a count for the occurrence of each word to the map
fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Hint: Use a for loop and `text.split_whitespace()` to iterate over the words.
    // Use `.entry()` and `.or_insert()` to update the count for each word.

    println!("{map:?}"); // Should print: {"hello": 1, "world": 2, "wonderful": 1}
}
