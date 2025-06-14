// Refactor this function to use a match expression
fn decr_twice(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
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
