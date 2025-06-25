// Solution: The code is modified to print messages showing the lifecycle of the boxed value.
fn main() {
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
