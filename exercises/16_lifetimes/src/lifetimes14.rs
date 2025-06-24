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

fn get_static_str() -> &str {
    // ✅ This is valid: the string literal has `'static` lifetime.
    "I live forever!"
}

// TODO: Fix this function by returning a reference that actually lives long enough
// ⚠️ WARNING: Adding `'static` here would be incorrect!
fn get_first_word(s: String) -> &str {
    let first = s.split_whitespace().next().unwrap();
    first
}

fn main() {
    let literal = get_static_str();
    println!("Static string: {literal}");

    let input = String::from("hello world");
    let word = get_first_word(input);
    println!("First word: {word}");
}
