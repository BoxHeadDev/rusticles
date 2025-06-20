// 🦀 Rustlings Challenge: Lifetime Elision
//
// This exercise will help you understand Rust's lifetime elision rules.
// Your task is to explicitly annotate the lifetimes in the `longest_explicit` function
// so that it compiles. Compare it to `longest_implicit`, which works due to elision rules.
//
// RULES:
// 1. Each parameter gets its own lifetime parameter.
// 2. If there's only one input lifetime, it gets assigned to the output.
// 3. If it's a method and one parameter is `&self` or `&mut self`, its lifetime is used for the output.

fn longest_implicit(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// TODO: Fix the function signature with proper lifetime annotations
// ❌ This won't compile until you annotate the lifetimes.
fn longest_explicit(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = String::from("short");
    let b = String::from("longer");

    let result = longest_implicit(&a, &b);
    println!("Longest (implicit): {}", result);

    let result = longest_explicit(&a, &b);
    println!("Longest (explicit): {}", result);
}
