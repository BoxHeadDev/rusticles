fn main() {
    // Create a populated vector of integers.
    let v2 = vec![1, 2, 3];

    // Assign the third element in the vector using indexing.
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    // Assign the third element in the vector using the `.get()` method.
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
// Context: Vectors in Rust allow you to access elements in two primary ways:
// Using indexing (e.g., v[2]) to directly access an element at a specific position.
// Using the .get() method, which returns an Option to handle cases where the index might be out of bounds.
