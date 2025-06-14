// Solution: The code is fixed by avoiding ownership transfer and instead using cloning if needed.
fn main() {
    let s = String::from("Hello world"); // `s` owns the string.
    let s_ref = &s; // Borrow `s` immutably.
    let s2 = s_ref.clone(); // Clone the string instead of moving ownership.
    println!("{s2}"); // Now `s2` has its own copy of the string, and `s` is still valid.
}
// Context: The original code attempts to take ownership of a string via dereferencing an immutable reference, leading to a potential double free issue. This happens because the ownership of the string is moved when you dereference s_ref, but s still exists in scope. Rust prevents this to ensure memory safety.
