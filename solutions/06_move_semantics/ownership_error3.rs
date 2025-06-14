// Solution: The code is fixed by borrowing only the length of the largest string, avoiding conflicts.
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len(); // Get the length only.
    for s in src {
        if s.len() > largest_len {
            // Compare the length without needing the full string reference.
            dst.push(s.clone()); // Safely mutate `dst` because there are no conflicting borrows.
        }
    }
}

fn main() {
    // Execute the function to see if your changes worked!
    let mut dst = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];

    let src = vec![
        String::from("kiwi"),
        String::from("pineapple"),
        String::from("grape"),
    ];

    add_big_strings(&mut dst, &src);
}

// Context: The original code creates an immutable reference to the largest string in a mutable vector (dst) while iterating and mutating the same vector. This violates Rust’s borrowing rules, which prevent simultaneous mutable and immutable borrows.
