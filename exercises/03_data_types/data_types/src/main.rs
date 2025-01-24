use std::io;

// Fix the code by adding the appropriate type annotation.
// Hint: The type should match a common unsigned integer type.
fn types_1() {
    let guess = "42".parse().expect("Not a number!");
}

// Add explicit type annotations to the variables below.
fn types_2() {
    let t = true; // Hint: t represents a boolean value.
    let f = false; // Hint: f is also a boolean value.
    let c = 'z'; // Hint: c represents a single character.
}

// Fix the integer overflow by using a method to handle overflow explicitly.
fn types_3() {
    let max: u8 = 255; // Hint: 255 is the maximum value for the u8 type.
    let total = max + 1; // This causes an integer overflow.

    println!("Value: {}", total);
}

// Specify the types for the following variables
fn types_4() {
    let x = 2.0; // Hint: What type will Rust infer for this variable?
    let y = 3.0; // Hint: Assign an explicit type to this variable.
}

// Correct the type annotation for `x` to fix the compiler error.
fn types_5() {
    let x: fsize = 2.0; // Hint: `fsize` is not a valid type. Replace it with a valid floating-point type.
    println!("{x}");
}

// Destructure the tuple into three variables: x, y, and z.
fn types_6() {
    let tup = (500, 6.4, 1); // Hint: Add explicit type annotations for the tuple elements.

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}

// fix the code by replacing invalid assignments with tuple element access using their respective indices.
fn types_7() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x; // Hint: Access the first element of the tuple.
    let six_point_four = x; // Hint: Access the second element of the tuple.
    let one = x; // Hint: Access the third element of the tuple.
}

// Fix the following array initialisation and access
fn types_8() {
    let a = [1, 2, 3, 4, 5]; // Hint: Add an explicit type annotation for the array.

    let b = [3, 3, 3, 3, 3]; // Hint: Use shorthand notation to initialise the array with repeated values.

    let first = a; // Hint: Access the first element of the array using indexing.
    let second = a; // Hint: Access the second element of the array using indexing.
}

// Ensure the program does not panic when a user enters an invalid index by adding proper index validation.
fn types_9() {
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

fn main() {
    // Execute the function to see if your changes worked!
    types_1();
    types_2();
    types_3();
    types_4();
    types_5();
    types_6();
    types_7();
    types_8();
    types_9();
}
