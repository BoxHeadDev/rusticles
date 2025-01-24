// Fix the code to ensure variables are properly declared and initialised before use.
// The goal is to correctly define `x` before passing it to the `read` function.
fn ownership_1() {
    read(x); // ERROR: `x` is used before being declared.
    let x = true; // `x` is declared here.
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

// Modify the code to demonstrate the lifecycle of a boxed value.
// The goal is to observe when and how the boxed value is dropped.
fn ownership_2() {
    let a_num = 4; // A simple stack-allocated integer.
    make_and_drop(); // A function that creates and drops a boxed value.
}

fn make_and_drop() {
    let a_box = Box::new(5); // Create a boxed value (allocated on the heap).
                             // Observe when `a_box` is dropped.
}

// Modify the code to demonstrate the ownership transfer between `a` and `b`.
// The goal is to show what happens when ownership of a `Box` is moved.
fn ownership_3() {
    let a = Box::new([0; 1_000_000]); // A heap-allocated array.
    let b = a; // Ownership of the array is moved from `a` to `b`.
               // Attempting to use `a` after this point will result in an error.
}

// Fix the code to avoid using `first` after its ownership is moved to `add_suffix`.
// The goal is to safely handle ownership while preserving access to the original string value.
fn ownership_4() {
    let first = String::from("Ferris"); // Create a new `String`.
    let full = add_suffix(first); // Ownership of `first` is moved here.
    println!("{full}, originally {first}"); // ERROR: `first` is no longer valid.
}

fn add_suffix(mut s: String) -> String {
    s.push_str(" world"); // Append a suffix to the string.
    s // Return the modified string.
}

// Fix the code to ensure proper cloning and ownership handling.
// The goal is to create a clone of `second` and use it safely.
fn ownership_5() {
    let second = String::from("Ferris"); // Create a new `String`.
    let second_clone = second.clone(); // Clone `second` to preserve its original value.
    let whole = add_suffix(first_clone); // ERROR: `first_clone` is not defined; use `second_clone`.
    println!("{whole}, originally {second}"); // Print both the modified and original strings.
}

fn add_suffix(mut s: String) -> String {
    s.push_str(" Rustacean"); // Append a suffix to the string.
    s // Return the modified string.
}

// Modify the code to ensure proper ownership transfer and understand how Rust handles returning values.
fn ownership_6() {
    let s = String::from("hello"); // Create a new `String`.
    let s2 = add_suffix(s); // Transfer ownership of `s` to `add_suffix`.
    println!("{}", s2); // Print the modified string.
}

fn add_suffix(mut s: String) -> String {
    s.push_str(" world"); // Append " world" to the string.
    s // Return the modified string, transferring ownership back to the caller.
}

// Fix the code to handle conditional ownership transfer safely.
// The goal is to ensure `s` and `s2` are properly managed without ambiguity.
fn ownership_7() {
    let s = String::from("hello"); // Create a new `String`.
    let s2; // Declare `s2` without initialisation.
    let b = false; // Set the condition.
    if b {
        s2 = s; // Move ownership of `s` to `s2` if `b` is true.
    }
    println!("{}", s); // ERROR: `s` might have been moved, so this is not allowed.
}

// Fix the code to handle ownership transfer properly.
// The goal is to ensure `Box` ownership is transferred safely without ambiguity.
fn ownership_8() {
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b; // Ownership of the Box is moved to `b2`.
    move_a_box(b); // ERROR: Ownership of `b` has already been moved, so this is invalid.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}

// Fix the code to handle ownership transfer properly.
// The goal is to ensure values are used safely and ownership is transferred without errors.
fn ownership_9() {
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b; // Ownership of the Box is moved to `b2`.
    println!("{}", b); // ERROR: `b` is no longer valid after ownership is moved to `b2`.
    move_a_box(b2); // Transfer ownership of `b2` to the function.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}

// Fix the code to handle ownership transfer properly.
// The goal is to ensure `Box` ownership is safely managed without using invalid variables.
fn ownership_10() {
    let b = Box::new(0); // Create a new Box containing 0.
    move_a_box(b); // Transfer ownership of `b` to the function.
    let b2 = b; // ERROR: `b` is no longer valid after its ownership was moved.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}

// Fix the code to avoid using a variable (`b`) after its ownership has been moved.
// The goal is to ensure ownership is handled safely without invalid references.
fn ownership_11() {
    let b = Box::new(0); // Create a new Box containing 0.
    move_a_box(b); // Transfer ownership of `b` to the function.
    println!("{}", b); // ERROR: `b` is no longer valid after its ownership was moved.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}

fn main() {
    // Execute the function to see if your changes worked!
    ownership_1();
    ownership_2();
    ownership_3();
    ownership_4();
    ownership_5();
    ownership_6();
    ownership_7();
    ownership_8();
    ownership_9();
    ownership_10();
    ownership_11();
}
