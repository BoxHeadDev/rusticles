// Fix the code to avoid conflicting borrows while mutating the `dst` vector.
// The goal is to ensure no immutable references are active while the vector is being mutated.
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // Immutable borrow of `dst`.
    for s in src {
        if s.len() > largest.len() {
            // ERROR: `dst` cannot be mutated while `largest` exists.
            dst.push(s.clone()); // Mutating `dst` here conflicts with the immutable borrow.
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

    add_big_strings();
}
