// Error: This code does not compile because you cannot borrow v mutably to iterate over it (for i in &mut v) and simultaneously mutate v by adding elements (v.push(*i)).
// Solution: To fix this, you can use a separate vector to store new elements and then extend the original vector after the iteration is complete.
fn main() {
    let mut v = vec![1, 2, 3];
    let mut new_elements = vec![];

    // Iterate over the vector and prepare new elements to add later.
    for i in &v {
        new_elements.push(*i);
    }

    // Extend the original vector with the new elements.
    v.extend(new_elements);

    // Safely access the new elements.
    println!("{} {} {}", v[3], v[4], v[5]); // Outputs: 1 2 3
}
// Context: In Rust, you cannot mutate a collection (e.g., adding elements to a vector) while iterating over it. This is because iteration borrows the collection immutably or mutably, and mutating the collection during iteration would violate Rust’s borrowing rules, potentially leading to undefined behaviour.
