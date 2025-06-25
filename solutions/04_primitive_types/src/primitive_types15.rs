// Solution: check if the index is within bound before accessing the array
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

    if index < a.len() {
        let element = a[index];
        println!("The value of the element at index {index} is: {element}");
    } else {
        println!("Index out of bounds! Please enter a valid index.");
    }
}
// Context: In Rust, arrays have a fixed size, and attempting to access an index outside the valid range will cause a runtime panic. This behaviour protects your program from accessing invalid memory. To prevent such panics, it’s important to validate the index before accessing the array.
