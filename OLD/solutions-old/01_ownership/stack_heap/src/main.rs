// Solution: The code is fixed by declaring `x` before it is used.
fn ownership_1() {
    let x = true; // Declare and initialise `x` before using it.
    read(x); // Pass `x` to the `read` function.
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}
// Context: The original code attempts to use the variable x before it is declared. Rust enforces strict rules ensuring variables are declared and initialised before use to prevent undefined behaviour.

// Solution: The code is modified to print messages showing the lifecycle of the boxed value.
fn ownership_2() {
    let a_num = 4; // A simple stack-allocated integer.
    println!("a_num is created: {a_num}");
    make_and_drop(); // A function that creates and drops a boxed value.
    println!("make_and_drop() has finished.");
}

fn make_and_drop() {
    println!("Creating a boxed value.");
    let a_box = Box::new(5); // Create a boxed value (allocated on the heap).
    println!("a_box is created with value: {a_box}");
    // When `a_box` goes out of scope here, it will be automatically dropped.
    println!("a_box is about to be dropped.");
}
// Context: The provided code creates a boxed integer in the make_and_drop function, but it doesn't demonstrate its behaviour when the box goes out of scope.

// Solution: The code is modified to safely demonstrate ownership transfer.
fn ownership_3() {
    let a = Box::new([0; 1_000_000]); // A heap-allocated array.
    println!("a is created, and it owns the array.");

    let b = a; // Ownership of the array is moved from `a` to `b`.
    println!("Ownership of the array is moved to b.");

    // `a` can no longer be used here; uncommenting the next line will cause a compile error:
    // println!("{:?}", a);

    // `b` now owns the array and can be used:
    println!("b now owns the array, and its length is {}", b.len());
}
// Context: The original code demonstrates that when a (a Box containing a large array) is assigned to b, ownership of the heap-allocated memory is moved to b. After this, a is no longer valid, and attempting to use a will result in a compilation error.

// Solution: The code is fixed by cloning `first` before passing ownership or reordering operations.
fn ownership_4() {
    let first = String::from("Ferris"); // Create a new `String`.
    let full = add_suffix(first.clone()); // Clone `first` to preserve its value.
    println!("{full}, originally {first}"); // Safely print both the original and modified strings.
}

fn add_suffix(mut s: String) -> String {
    s.push_str(" world"); // Append a suffix to the string.
    s // Return the modified string.
}
// Context: The original code attempts to use the variable first after its ownership has been moved to the add_suffix function. Rust prevents this because once ownership is transferred, the original variable is no longer valid.

// Solution: The code is fixed to use `second_clone` correctly.
fn ownership_5() {
    let second = String::from("Ferris"); // Create a new `String`.
    let second_clone = second.clone(); // Clone `second` to preserve the original value.
    let whole = add_suffix2(second_clone); // Pass the cloned value to `add_suffix`.
    println!("{whole}, originally {second}"); // Safely print both the modified and original strings.
}

fn add_suffix2(mut s: String) -> String {
    s.push_str(" Rustacean"); // Append a suffix to the string.
    s // Return the modified string.
}
// Context: The original code uses .clone() to create a duplicate of a String value (second) so that the cloned version can be passed to a function (add_suffix) without affecting the original string. This ensures the original string remains valid and accessible after the function call.

// Solution:
fn ownership_6() {
    let s = String::from("hello"); // Create a new `String`, owned by `s`.
    let s2 = add_suffix3(s); // Ownership of `s` is moved to `add_suffix`.
                             // `s` is no longer valid here; `s2` now owns the modified string.
    println!("{}", s2); // Prints "hello world".
}

fn add_suffix3(mut s: String) -> String {
    s.push_str(" world"); // Modify the string by appending " world".
    s // Return the modified string, transferring ownership back to the caller.
}
// Context: The function add_suffix takes ownership of a String, modifies it by appending text, and then returns the modified String. The original string (s) is moved to add_suffix, and its ownership is transferred back to the caller as s2.

// Solution:
fn ownership_7() {
    // Solution 1: Avoid ownership transfer
    let s = String::from("hello"); // Create a new `String`.
    let b = false; // Set the condition.
    if b {
        // Do something with `s` or `s2`, but do not transfer ownership.
        println!("{}", s); // Safely use `s` as its ownership is not transferred.
    }
    println!("{}", s); // `s` is always valid here.

    // Solution 2: Use Cloning for Safety
    let s = String::from("hello"); // Create a new `String`.
    let mut s2 = String::new(); // Initialise `s2` to avoid ambiguity.
    let b = false; // Set the condition.
    if b {
        s2 = s.clone(); // Clone `s` to ensure `s` remains valid.
    }
    println!("{}", s); // Safely use `s`.
    if b {
        println!("{}", s2); // Safely use `s2` if assigned.
    }

    // Solution 3: Reassign Ownership Unconditionally
    let s = String::from("hello"); // Create a new `String`.
    let s2 = if false {
        s // Move ownership of `s` if the condition is true.
    } else {
        String::from("") // Provide a fallback `String` when the condition is false.
    };
    println!("{}", s2); // Safely use `s2`, now fully assigned.
}
// Context: The original code attempts to move ownership of s into s2 inside a conditional block. However, since the condition is false, s2 is not assigned, and s is used afterward. Rust ensures that variables involved in such ownership transfers cannot be accessed if there’s ambiguity about their validity.

// Solution:
fn ownership_8() {
    // Solution 1: Use Only b2 After Ownership Transfer
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b; // Ownership of the Box is moved to `b2`.
    move_a_box(b2); // Use `b2` to transfer ownership.

    // Solution 2: Avoid Transferring Ownership to b2
    let b = Box::new(0); // Create a new Box containing 0.
    move_a_box(b); // Transfer ownership of `b` directly without using `b2`.

    // Solution 3: Clone the Box to Keep Ownership
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b.clone(); // Clone the Box to keep both `b` and `b2` valid.
    move_a_box(b); // Transfer ownership of `b` to the function.
    println!("Still have b2: {}", b2); // Use the cloned Box.
}

fn move_a_box(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}
// Context: The original code transfers ownership of a Box value to two places: first to b2 and then to the move_a_box function. After ownership is moved, the original variable (b) is no longer valid. Rust enforces these rules to prevent undefined behaviour, such as double-free errors.

// Solution:
fn ownership_9() {
    // Solution 1: Use Only b2 After Ownership Transfer
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b; // Ownership of the Box is moved to `b2`.
    println!("{}", b2); // Use `b2`, which now owns the Box.
    move_a_box2(b2); // Transfer ownership of `b2` to the function.

    // Solution 2: Avoid Transferring Ownership to b2
    let b = Box::new(0); // Create a new Box containing 0.
    println!("{}", b); // Use `b` before transferring ownership.
    move_a_box2(b); // Transfer ownership of `b` directly to the function.

    // Solution 3: Clone the Box if You Need Both b and b2
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b.clone(); // Clone the Box to preserve the original value in `b`.
    println!("{}", b); // Use the original Box.
    move_a_box2(b2); // Transfer ownership of the cloned Box to the function.
}

fn move_a_box2(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}
// Context: The original code transfers ownership of a Box from b to b2, but it attempts to use b afterward. This is not allowed in Rust because b is no longer valid after the ownership is moved to b2.

// Solution:
fn ownership_10() {
    // Solution 1: Use a New Variable Instead of b2
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b.clone(); // Clone the Box to preserve the original value in `b`.
    move_a_box3(b2); // Transfer ownership of the cloned Box to the function.
    println!("Original Box value: {}", b); // Safely use `b` since it wasn’t moved.

    // Solution 2: Avoid Reassigning Ownership to b2let b = Box::new(0); // Create a new Box containing 0.
    move_a_box3(b); // Transfer ownership of `b` to the function.
                    // `b` is no longer valid here, so no need to assign it to `b2`.

    // Solution 3: Reassign Ownership Without Calling move_a_box Directly
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b; // Reassign ownership of `b` to `b2`.
    println!("Reassigned Box value: {}", b2); // Safely use `b2`.
    move_a_box3(b2); // Now transfer ownership of `b2` to the function.
}

fn move_a_box3(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}
// Context: The original code transfers ownership of a Box from b to the move_a_box function, then attempts to use b again to assign it to b2. This is not allowed because b is no longer valid after its ownership has been moved.

// Solution:
fn ownership_11() {
    // Solution 1:
    let b = Box::new(0); // Create a new Box containing 0.
    move_a_box4(b); // Transfer ownership of `b` to the function.
                    // No attempt to use `b` here, as it is no longer valid.

    // Solution 2: Clone b Before Transferring Ownership
    let b = Box::new(0); // Create a new Box containing 0.
    let b_clone = b.clone(); // Clone the Box to preserve the original value.
    move_a_box4(b); // Transfer ownership of the original `b` to the function.
    println!("{}", b_clone); // Safely use the cloned Box.

    // Solution 3: Reassign Ownership to a New Variable
    let b = Box::new(0); // Create a new Box containing 0.
    let b2 = b; // Move ownership of `b` to `b2`.
    move_a_box4(b2); // Transfer ownership of `b2` to the function.
                     // `b` is not used here, as ownership has been moved.
}

fn move_a_box4(b: Box<i32>) {
    println!("Moved value: {}", b); // Ownership of the Box is taken here.
}
// Context: The original code transfers ownership of a Box from b to the move_a_box function, then attempts to use b afterward. This is not allowed because b is no longer valid after its ownership has been moved.

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
