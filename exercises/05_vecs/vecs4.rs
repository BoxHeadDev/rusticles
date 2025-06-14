fn main() {
    // Create a populated vector of integers.
    let v2 = ??; // Hint: Use the `vec!` macro.

    // Assign the third element in the vector using indexing.
    let third = ??; // Hint: Use indexing (e.g., `v2[2]`).
    println!("The third element is {third}");

    // Assign the third element in the vector using the `.get()` method.
    let third = ??; // Hint: Use `v2.get(2)` to retrieve an `Option`.
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
