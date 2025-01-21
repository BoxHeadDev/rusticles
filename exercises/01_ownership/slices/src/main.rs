fn main() {
    // Fix bug
    bug_example();

    // Examples
    slice();
    range();
}

// Tracking indexes is tedious and error prone
// string and space index integer are not linked
fn bug_example() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // string can be changed
}

fn first_word(s: &String) -> usize {
    // convert string to array of bytes
    let bytes = s.as_bytes();

    // create iterator over array of bytes
    // enumerate creates a tuple (index, element) which can be destructured
    for (i, &item) in bytes.iter().enumerate() {
        // Loook for space using byte literal
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Reference word using a slice
fn slice() {
    let mut s = String::from("hello");
    let hello: &str = &s[0..5]; // slice
    println!("{hello}");
    s.push_str(" world");
}

fn range() {
    let s = String::from("hello");

    let len = s.len();

    // reference part of string
    let slice = &s[3..len];
    let slice = &s[3..];

    // reference whole string
    let slice = &s[0..len];
    let slice = &s[..];
}
