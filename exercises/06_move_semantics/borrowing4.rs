// Fix this code to make it compile by respecting Rust's borrowing rules.
// The goal is to handle borrowing and mutation correctly.
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2]; // ERROR: Immutable borrow while `v` might be mutated.
    v.push(4); // ERROR: Cannot mutate `v` while an immutable reference exists.
    println!("Third element is {}", *num); // ERROR: `num` may have been invalidated.
}
