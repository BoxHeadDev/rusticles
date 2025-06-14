fn main() {
    // What is wrong with the following code?
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // Immutable reference to the first element.
    v.push(6); // Mutable borrow to add an element to the vector.

    println!("The first element is: {first}");
}
