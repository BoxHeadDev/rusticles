// Fix the code to prevent modifying the string while using its word reference.
// The goal is to link the string and its first word more safely.
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // Get the index of the first word.
    s.clear(); // ERROR: The string can be modified, making the index invalid.
    println!("The first word is at index: {}", word);
}

fn first_word(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
