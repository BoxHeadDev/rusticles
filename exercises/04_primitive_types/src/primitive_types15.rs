// Ensure the program does not panic when a user enters an invalid index by adding proper index validation.
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // Hint: Add a check to ensure the index is within bounds
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
