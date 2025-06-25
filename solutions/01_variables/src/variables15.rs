// A variable cannot be assigned to a value of a different type

// Solution 1: Correct the type mismatch by assigning a value of the same type
fn shadowing_4_a() {
    let mut x: u32 = 1; // Declare `x` as a u32
    x = 2; // Assign another value of type u32
    println!("{x}"); // Prints the updated value of `x` (2)
}

// Solution 2: Use shadowing to reuse the variable name with a new type
fn shadowing_4_b() {
    let x: u32 = 1; // Declare `x` as a u32
    let x = "Hello world"; // Shadow `x` with a new variable of type &str
    println!("{x}"); // Prints "Hello world"
}
// Context: In Rust, a variable's type is fixed when it is declared. You cannot assign a value of a different type to the same variable, even if it is mutable. If you need to use the same variable name with a different type, you can use shadowing instead.
