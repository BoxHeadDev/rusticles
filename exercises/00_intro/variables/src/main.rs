
// Fix the error in the code below by allowing the variable to be mutable
fn variables_1() {
    let x = 5; // The variable `x` is currently immutable.
    println!("The value of x is: {x}");
    x = 6; // This will cause a compilation error. How can you fix it?
    println!("The value of x is: {x}");
}

// Fix the error in the code by allowing the variable to be mutable
fn variables_2() {
    let x = 1; // `x` is currently immutable
    println!("{x}"); // Prints the initial value of `x`
    x += 1; // This will cause a compilation error. How can you fix it?
    println!("{x}"); // Should print the updated value of `x`
}

// Fix the error by adding a type annotation to the constant
fn constants_1() {
    const THREE_HOURS_IN_SECONDS = 60 * 60 * 3; // This constant is missing a type annotation
    println!(THREE_HOURS_IN_SECONDS); // Should print the value of the constant
}

// Fix the code so that it compiles and follows Rust's rules for constants
fn constants_2() {
    const mut TOTAL = 50; // Constants cannot be mutable
    println!(TOTAL); // Prints the initial value of TOTAL
    TOTAL = 100; // This will cause a compilation error. What can you do instead?
    println!(TOTAL); // Prints the new value of TOTAL
}

// Fix the code so that the constant is accessible and works as expected
fn constants_3() {
    println!("{THREE}"); // Should print the value of the constant
}

// Fix the code to correctly demonstrate shadowing and scopes
fn shadowing_1() {
    let x = 5;
    let x = x + 1; // Shadow the original `x`
    let x = x * 2; // Shadow `x` again in the inner scope

    println!("The value of x in the inner scope is: {x}"); // Should print 12
    println!("The value of x is: {x}"); // Should print 6
}

// Fix the code to use shadowing instead of mutability
fn shadowing_2() {
    let mut spaces = "   "; // This variable is mutable, but we don't need mutability here
    spaces = spaces.len(); // Transforming `spaces` from a string to its length causes a type mismatch
}

// Fix or explain the behavior of the code
fn shadowing_3() {
    let mut x: u32 = 1; // Outer variable `x` is mutable
    {
        let mut x = x; // Shadow the outer `x` with a new mutable variable
        x += 2; // Modify the inner `x`
    }
    println!("{x}"); // What value will this print? Fix or explain!
}

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

fn main() {
    // Execute the function to see if your changes worked! 
    variables_1();
    variables_2();
  
    constants_1();
    constants_2();
    constants_3();
  
    shadowing_1();
    shadowing_2();
    shadowing_3();
    shadowing_4_a();
    shadowing_4_b();
}
