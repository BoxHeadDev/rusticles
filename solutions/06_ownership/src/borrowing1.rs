// Solution: This version fixes the ownership issues by using references.
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // Borrow `m1` and `m2` instead of transferring ownership.
    let s = format!("{} {}", m1, m2); // `m1` and `m2` are still accessible here!
}

fn greet(g1: &String, g2: &String) {
    // Accept references instead of taking ownership of `g1` and `g2`.
    println!("{} {}!", g1, g2);
}
// Context: Borrowing allows you to pass references to functions without transferring ownership, enabling you to reuse variables after the function call.
