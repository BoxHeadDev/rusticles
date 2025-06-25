// Solution: Refactored function using a match expression
fn decr_twice(n: u32) -> Option<u32> {
    match n {
        0 => None,          // If n is 0, return None
        1 => None,          // If n is 1, return None
        n2 => Some(n2 - 2), // For any other value, return n - 2 wrapped in Some
    }
}

fn main() {
    decr_twice();

    assert_eq!(decr_twice(0), None);
    assert_eq!(decr_twice(1), None);
    assert_eq!(decr_twice(2), Some(0));
    assert_eq!(decr_twice(5), Some(3));
    println!("All tests passed!");
}
// Context: This approach is more idiomatic in Rust because match clearly shows the branching logic based on the value of n.
