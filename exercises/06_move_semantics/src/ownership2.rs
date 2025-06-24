// Modify the code to demonstrate the lifecycle of a boxed value.
// The goal is to observe when and how the boxed value is dropped.
fn main() {
    let a_num = 4; // A simple stack-allocated integer.
    make_and_drop(); // A function that creates and drops a boxed value.
}

fn make_and_drop() {
    let a_box = Box::new(5); // Create a boxed value (allocated on the heap).
    // Observe when `a_box` is dropped.
}
