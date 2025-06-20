// 🦀 Rustlings Challenge: Mixed Mutability with Lifetimes
//
// You are writing a function that takes:
// - one **immutable** reference to a `String`
// - one **mutable** reference to a `String`
//
// Your goal is to return a **mutable** reference to the longer string.
// This will only work if the longer string is the mutable one,
// because Rust does not allow mutably borrowing something that's immutably borrowed.
//
// ✅ You'll need lifetime annotations for both references.
// ❌ Do NOT try to coerce an immutable reference into a mutable one (Rust forbids it!)

fn longer_mutable<'a>(x: &'a String, y: &'a mut String) -> &'a mut String {
    if x.len() > y.len() {
        // ✅ x is immutable; we cannot return it mutably
        // ✅ Instead, return y even if it's shorter — safe and legal
        println!("Warning: mutable string was shorter, returning it anyway");
        y
    } else {
        y
    }
}

fn main() {
    let a = String::from("short");
    let mut b = String::from("a bit longer");

    let result = longer_mutable(&a, &mut b);
    result.push_str(" (updated)");
    println!("{result}");
}
