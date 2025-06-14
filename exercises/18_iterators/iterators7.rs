fn main() {
    // Iterate over a vector without using a pointer.
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: ? = ??; // Hint: Use a range of indices (0..v.len()).

    let i1: usize = iter.next().unwrap(); // Retrieve the first index.
    let n1: &i32 = &v[i1];               // Access the element at the index.
}
