fn main() {
    // Iterate over a vector without using a pointer.
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: std::ops::Range<usize> = 0..v.len(); // Create a range of indices.

    let i1: usize = iter.next().unwrap(); // Retrieve the first index (0).
    let n1: &i32 = &v[i1]; // Access the element at the index.

    println!("Index: {i1}, Value: {n1}"); // Outputs: Index: 0, Value: 1
}
// Context: Instead of using .iter() to borrow references to the elements of a vector, you can iterate over the indices of a vector using a Range (e.g., 0..v.len()). This allows you to access elements directly via indexing, which can be useful when you need to work with indices explicitly.
