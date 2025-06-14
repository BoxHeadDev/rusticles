// Solution: The issue can be fixed by returning a string slice (&str) instead of an index, which ties the word to the String and prevents modifying the String while the slice is borrowed.
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // Get a string slice representing the first word.
    // s.clear(); // ERROR: Cannot modify `s` while `word` exists.
    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // Return a slice of the string up to the first space.
        }
    }
    &s[..] // If no space is found, return the entire string.
}
// Context: The original code demonstrates a common pitfall: storing the result of a function (first_word) that returns an index into a String, then modifying the String. This leads to a disconnect between the String and its associated index, which Rust does not automatically track.
