// Fix the code to avoid conflicts with mutable references and ensure safe access to `point`.
// The goal is to safely modify and print the elements of the array.
fn main() {
    let mut point = [0, 1];
    let mut x = point[0]; // Immutable copy of `point[0]`.
    let y = &mut point[1]; // Mutable borrow of `point[1]`.
    x += 1; // Increment the copy of `point[0]`.
    *y += 1; // ERROR: Cannot have a mutable borrow while `point` is still accessed.
    println!("{} {}", point[0], point[1]); // ERROR: Conflicting access to `point`.
}
