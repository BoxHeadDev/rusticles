fn main() {
    // What is the output?
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        v2.push(i); // Push mutable references to `v` into `v2`.
    }

    *v2[0] = 5; // Modify the first element of `v` through the mutable reference in `v2`.
    let a = *v2[0]; // Dereference the first mutable reference to get its value.
    let b = v[0]; // Access the first element of `v` directly.

    println!("{a} {b}"); // Outputs: 5 5
}
// Context: In Rust, you can create a vector of mutable references to the elements of another vector. When you modify an element through the mutable reference, the changes are reflected in the original vector. This demonstrates how Rust ensures memory safety while allowing powerful, low-level operations.
