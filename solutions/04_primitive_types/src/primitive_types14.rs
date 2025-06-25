// Solution:
fn main() {
    // Adding an explicit type annotation for the array: `[i32; 5]` means an array of 5 `i32` values.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Using shorthand notation `[value; count]` to create an array of repeated values.
    let b = [3; 5];

    // Accessing specific elements of the array using indexing.
    let first = a[0]; // The first element (1) is at index 0.
    let second = a[1]; // The second element (2) is at index 1.
}
// Context: Arrays in Rust are a fixed-size collection of elements, where all elements must have the same type. You can declare arrays with explicit type annotations or use shorthand for repeated values. Additionally, you can access array elements using indexing, starting from 0 for the first element.
