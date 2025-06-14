use std::io;

// Solution: Explicitly specifying the type ensures the compiler knows what type to parse the string into. In this case, we use u32.
fn types_1() {
    let guess: u32 = "42".parse().expect("Not a number!");
}
// Context: The Rust compiler needs to know the specific type of a variable when parsing a string into a number, as there are multiple numeric types in Rust (e.g., u32, i32, f64). Without an explicit type, the code will fail to compile.

// Solution: Adding explicit type annotations makes the variable types clear and helps the compiler catch potential type-related errors.
fn types_2() {
    let t: bool = true; // `bool` is the type for boolean values.
    let f: bool = false; // Explicitly specifying `bool` for false.
    let c: char = 'z'; // `char` is the type for a single Unicode character.
}
// Context: In Rust, the compiler can often infer the types of variables, but there are times when adding explicit type annotations improves code clarity and prevents ambiguity. Explicit annotations make it easier for others (and the compiler) to understand the type of each variable, especially in more complex scenarios.

// Solution: Using `wrapping_add` ensures the value wraps around to 0 instead of causing a panic.
fn types_3() {
    let max: u8 = 255;
    let total = max.wrapping_add(1);

    println!("Value: {}", total); // This will now print "Value: 0".
}
// Context: Rust’s numeric types, such as u8, have a fixed range of values. For example, a u8 can hold values between 0 and 255. If an operation exceeds this range, it results in an integer overflow. By default, Rust will panic in debug mode when an overflow occurs. However, you can handle this explicitly using methods like wrapping_add, which wraps the value back to the start of the range instead of panicking.

// Solution: floating-point types are infered and defined
fn types_4() {
    let x = 2.0; // By default, Rust infers `f64` for floating-point literals.
    let y: f32 = 3.0; // Explicitly specify `f32` to use a 32-bit floating-point type.
}
// Context: Rust provides two floating-point types: f32 and f64. By default, Rust infers floating-point numbers as f64 for better precision. However, in some cases, you might want to use f32 explicitly to optimise for memory usage or performance.

// Solution: The type `fsize` does not exist in Rust. Floats must be either `f32` or `f64`.
fn types_5() {
    let x: f32 = 2.0; // Correctly specify `f32` as the floating-point type.
    println!("{x}");
}
// Context: Rust supports two floating-point types: f32 and f64. These represent 32-bit and 64-bit floating-point numbers, respectively. If you attempt to use an unsupported type like fsize, the compiler will throw an error, as it doesn’t exist.

// Solution: Add explicit type annotations to a tuple and destructure the tuple into separate variables to print each value.
fn types_6() {
    // Adding explicit type annotations makes the tuple's structure clear.
    let tup: (i32, f32, u8) = (500, 6.4, 1);

    // Destructuring the tuple into individual variables.
    let (x, y, z) = tup;

    println!("The value of x is: {x}"); // Prints: The value of x is: 500
    println!("The value of y is: {y}"); // Prints: The value of y is: 6.4
    println!("The value of z is: {z}"); // Prints: The value of z is: 1
}
// Context: Tuples in Rust can hold multiple values of different types. By default, Rust can infer the types of tuple elements, but adding explicit type annotations improves code clarity. Tuples can also be destructured into individual variables for convenient access to their elements.

// Solution: Access tuple elements using indices.
fn types_7() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // The first element (500) is at index 0.
    let six_point_four = x.1; // The second element (6.4) is at index 1.
    let one = x.2; // The third element (1) is at index 2.
}
// Context: In Rust, tuples allow you to store multiple values of different types in a single structure. To access individual elements of a tuple, you can use dot notation with an index (e.g., .0, .1, etc.). This is particularly useful when you don't want to destructure the tuple into variables but need specific values.

// Solution:
fn types_8() {
    // Adding an explicit type annotation for the array: `[i32; 5]` means an array of 5 `i32` values.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Using shorthand notation `[value; count]` to create an array of repeated values.
    let b = [3; 5];

    // Accessing specific elements of the array using indexing.
    let first = a[0]; // The first element (1) is at index 0.
    let second = a[1]; // The second element (2) is at index 1.
}
// Context: Arrays in Rust are a fixed-size collection of elements, where all elements must have the same type. You can declare arrays with explicit type annotations or use shorthand for repeated values. Additionally, you can access array elements using indexing, starting from 0 for the first element.

// Solution: check if the index is within bound before accessing the array
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

    if index < a.len() {
        let element = a[index];
        println!("The value of the element at index {index} is: {element}");
    } else {
        println!("Index out of bounds! Please enter a valid index.");
    }
}
// Context: In Rust, arrays have a fixed size, and attempting to access an index outside the valid range will cause a runtime panic. This behaviour protects your program from accessing invalid memory. To prevent such panics, it’s important to validate the index before accessing the array.

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
