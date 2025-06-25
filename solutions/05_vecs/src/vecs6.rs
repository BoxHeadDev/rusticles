fn main() {
    // Rust prevents mutable and immutable references from coexisting.
    let mut v = vec![1, 2, 3, 4, 5];

    // Resolve by limiting the scope of the immutable reference.
    let first = &v[0]; // Immutable reference to the first element.
    println!("The first element is: {first}");

    v.push(6); // Mutable borrow to add an element to the vector.
}
// Context: In Rust, you cannot have a mutable reference (&mut) and an immutable reference (&) to the same data in the same scope. This ensures memory safety and prevents data races. In this challenge, you’ll fix a code snippet that violates Rust’s borrowing rules when modifying a vector while holding an immutable reference.
