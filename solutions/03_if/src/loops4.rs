// Solution: Use a `while` loop to iterate through the array
fn main() {
    let a = [10, 20, 30, 40, 50]; // Define the array
    let mut index = 0; // Initialize the index variable

    while index < 5 {
        // Continue looping as long as the index is less than 5
        println!("the value is: {}", a[index]); // Print the value at the current index

        index += 1; // Increment the index to move to the next element
    }
}
// Context: In Rust, you can use a while loop to iterate through a collection, such as an array, by incrementing an index variable.
