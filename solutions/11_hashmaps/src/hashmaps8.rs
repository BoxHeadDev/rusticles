// Solution:
fn main() {
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
