// Fix the code to avoid double free errors while working with string ownership.
fn main() {
    let s = String::from("Hello world"); // `s` owns the string.
    let s_ref = &s; // Borrow `s` immutably.
    let s2 = *s_ref; // ERROR: This moves the ownership of `s` from `s_ref` to `s2`.
    println!("{s2}"); // `s` is no longer accessible here, but we still try to use it.
}
