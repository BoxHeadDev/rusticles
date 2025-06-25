// Fix the code to avoid ownership and double-free errors.
// The goal is to use the reference without transferring ownership.
fn main() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0]; // Immutable reference to the first element of the vector.
    let s: String = *s_ref; // ERROR: Cannot take ownership of the value through a reference.
}
