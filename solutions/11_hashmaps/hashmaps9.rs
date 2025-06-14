// This code won't compile as is because Rust prevents mutable and immutable borrows simultaneously.
fn main() {
    let mut h = HashMap::new();
    h.insert("k1", 0);

    // To fix this, we drop the immutable reference (v1) before mutating h.
    let v1 = h["k1"]; // Take a copy of the value instead of an immutable reference.

    h.insert("k2", 1); // Now we can safely mutate h.

    let v2 = h["k2"]; // Take another copy of the value.
    println!("{} {}", v1, v2); // Prints: "0 1"
}
// Context: In Rust, borrowing rules ensure memory safety by preventing mutable and immutable references to the same data at the same time. When working with a HashMap, you cannot modify it while an immutable reference to one of its elements is still active.
