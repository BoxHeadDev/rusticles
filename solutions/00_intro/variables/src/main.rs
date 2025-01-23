// Solution: Add the `mut` keyword to make the variable mutable
fn variables_1() {
    let mut x = 5; // Use `mut` to allow `x` to be modified
    println!("The value of x is: {x}"); // Prints the initial value of `x`
    x = 6; // Now that `x` is mutable, this line will work
    println!("The value of x is: {x}"); // Prints the updated value of `x`
}
// Context: In Rust, variables are immutable by default. This means once you assign a value to a variable, it cannot be changed. To allow modification of a variable's value, you need to explicitly declare it as mutable using the mut keyword.

// Solution: Add the `mut` keyword and ensure `x` is mutable
fn variables_2() {
    let mut x = 1; // Use `mut` to allow `x` to be modified
    println!("{x}"); // Prints the initial value of `x`
    x += 1; // Increment the value of `x` by 1
    println!("{x}"); // Prints the updated value of `x`
}
// Context: In Rust, variables are immutable by default. This means once you assign a value to a variable, it cannot be changed. To allow modification of a variable's value, you need to explicitly declare it as mutable using the mut keyword.

// Solution: Add a type annotation to the constant
fn constants_1() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Annotate the constant with type `u32`
    println!(THREE_HOURS_IN_SECONDS); // Prints the value of the constant
}
// Context: In Rust, constants are defined using the const keyword, and their types must always be explicitly annotated. Unlike variables, constants are evaluated at compile time and cannot be changed during the program's execution. In this exercise, your task is to add the correct type annotation to a constant to fix the compilation error.

// Solution: Remove mutability and use separate constants for different values
fn constants_2() {
    const TOTAL: u32 = 50; // Declare a constant with an explicit type annotation
    println!(TOTAL); // Prints the value of TOTAL

    const NEW_TOTAL: u32 = 100; // Use a new constant for the updated value
    println!(NEW_TOTAL); // Prints the value of NEW_TOTAL
}
// Context: In Rust, constants declared with the const keyword are always immutable. Unlike variables marked with let, constants cannot be changed after they are declared. Additionally, constants do not support the mut keyword, and attempting to modify them will result in a compilation error.

// Solution: Declare the constant with global scope
const THREE: u32 = 1 + 2; // A global constant that is accessible from anywhere in the program

fn constants_3() {
    println!("{THREE}"); // Prints the value of the constant (3)
}
// Context: In Rust, constants declared with the const keyword can have a global scope, making them accessible from any part of your program. Unlike variables declared inside functions, constants are evaluated at compile time and remain immutable throughout the program's execution.

// Solution: The code demonstrates shadowing and scopes effectively
fn shadowing_1() {
    let x = 5; // Initial value of `x`

    let x = x + 1; // Shadow `x` with a new value (5 + 1 = 6)

    {
        let x = x * 2; // Shadow `x` again in the inner scope (6 * 2 = 12)
        println!("The value of x in the inner scope is: {x}"); // Prints 12
    }

    println!("The value of x is: {x}"); // Prints 6 (the outer `x` is unaffected by the inner scope)
}
// Context: Rust allows shadowing, where you can declare a new variable with the same name as a previous variable. This feature allows you to reuse variable names without mutability, enabling transformations of values while keeping them immutable. Shadowing also respects scope boundaries, meaning changes in an inner scope do not affect the outer scope.

// Solution: Use shadowing to create a new variable with the same name
fn shadowing_2() {
    let spaces = "   "; // Declare `spaces` as a string
    let spaces = spaces.len(); // Shadow `spaces` to store its length as an integer
    println!("The length of spaces is: {spaces}"); // Prints the length of the string
}
// Context: In Rust, shadowing allows you to reuse the same variable name with a new value or type, without needing to declare it as mut. This is different from mutability, where the same variable is modified in place. Shadowing is useful for transforming data while keeping immutability in your program.

// Solution: The code is correct as-is, demonstrating shadowing and scope isolation
fn shadowing_3() {
    let mut x: u32 = 1; // Outer mutable variable `x`
    {
        let mut x = x; // Shadow the outer `x` with a new mutable variable
        x += 2; // Modify the inner `x` (this does not affect the outer `x`)
    }
    println!("{x}"); // Prints 1 because the outer `x` remains unchanged
}
// Context: In Rust, shadowing allows you to create a new variable with the same name as an existing one, even inside inner scopes. However, shadowing in an inner scope does not affect the value of the outer variable. This exercise helps you understand how shadowing works with scope isolation and demonstrates that changes made in an inner scope do not "leak" to the outer scope.

// A variable cannot be assigned to a value of a different type
fn shadowing_4() {
    let mut x: u32 = 1;
    x = 2;
    println!("{x}");
}
// Context: In Rust, a variable's type is fixed when it is declared. You cannot assign a value of a different type to the same variable, even if it is mutable. If you need to use the same variable name with a different type, you can use shadowing instead.

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
}
