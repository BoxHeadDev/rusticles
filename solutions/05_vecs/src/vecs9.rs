fn main() {
    let v = vec![String::from("Hello ")];

    // Solution 1: Fix by borrowing the value from the vector.
    let s = &v[0]; // Borrow the value immutably.
    let mut s = s.clone(); // Clone the borrowed value to make it mutable.

    // Solution 2: Fix by cloning the value directly.
    let mut s = v[0].clone(); // Clone the value to move it out safely.

    s.push_str("world");
    println!("{s}");
}
// Context: In Rust, elements of a vector are owned by the vector, and indexing directly (v[0]) tries to move the value out of the vector. This is not allowed for types like String, which are non-copyable. Instead, you must use .get() with references or explicitly clone the value if you want to work with it outside the vector.
