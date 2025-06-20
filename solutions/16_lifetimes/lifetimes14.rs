// 🦀 Rustlings Challenge: The Static Lifetime
//
// In this exercise, you'll learn when to use the `'static` lifetime and when *not* to.
//
// Your goal is to correctly annotate the lifetimes for each string reference,
// using `'static` only when it makes sense.
//
// ⚠️ DO NOT just add `'static` to make the compiler happy — use it only where valid.
//
// Questions to ask yourself:
// - Is this a string literal?
// - Does the reference truly live for the entire program?
// - Would it be better to borrow from existing data instead?

fn get_static_str() -> &'static str {
    // ✅ This is valid: the string literal has `'static` lifetime.
    "I live forever!"
}

// ✅ Accept a reference instead of taking ownership of the String.
// This ensures the borrowed string lives long enough for the returned slice.
fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap()
}

fn main() {
    let literal = get_static_str();
    println!("Static string: {literal}");

    let input = String::from("hello world");
    let word = get_first_word(&input); // ✅ Now we're borrowing input
    println!("First word: {word}");
}
