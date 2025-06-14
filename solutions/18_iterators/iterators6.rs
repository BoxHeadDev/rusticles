fn main() {
    // What are the values of the following iterator?
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();

    let n1: &i32 = iter.next().unwrap(); // Returns the first element: 1
    let n2: &i32 = iter.next().unwrap(); // Returns the second element: 2
    let end: Option<&i32> = iter.next(); // Returns None, as there are no more elements.

    println!("n1: {n1}, n2: {n2}, end: {:?}", end); // Outputs: n1: 1, n2: 2, end: None
}
// Context: In Rust, an iterator allows you to sequentially access elements in a collection without needing to manage indexing or bounds manually. Calling .next() on an iterator yields the next item in the collection or None if the iterator has reached the end. Iterators are a core concept in Rust, enabling functional programming patterns and efficient data manipulation.
