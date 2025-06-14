// The issue arises because:
// v.iter() creates an immutable borrow of v for iteration.
// v.push(*n_ref) attempts to mutate v while it’s still immutably borrowed, which violates Rust's borrowing rules.
fn main() {
    fn dup_in_place(v: &mut Vec<i32>) {
        // Solution 1: Use a Temporary Copy for Iteration
        let original = v.clone(); // Clone the vector to iterate over a temporary copy.
        for n in original.iter() {
            v.push(*n); // Mutably modify the original vector.
        }

        // Solution 2: Use an Index-Based Loop
        let len = v.len(); // Get the original length of the vector.
        for i in 0..len {
            v.push(v[i]); // Access elements by index and push them to the vector.
        }
    }
}
// Context: In Rust, iterating over a collection with .iter() borrows the collection immutably (&), which removes the ability to modify it within the same scope. This ensures that Rust prevents mutable and immutable borrows of the same data simultaneously, maintaining memory safety.
