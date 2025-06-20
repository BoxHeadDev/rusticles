// 🦀 Rustlings Challenge: Generic Type Parameters, Trait Bounds, and Lifetimes
//
// You're working with a function that returns the longer of two string slices
// and announces something using a generic parameter that implements `Display`.
//
// Your task is to fix the function signature:
// - Add the appropriate lifetime parameter `'a`
// - Add a generic type parameter `T` with a `Display` trait bound
// - Use a `where` clause to cleanly specify the trait bound
//
// HINT: The function returns a reference tied to the lifetime of `x` and `y`.

use std::fmt::Display;

// ❌ This won't compile until you fix the lifetime and trait annotations.
fn longest_with_announcement(x: &str, y: &str, ann: T) -> &str {
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("a little bit longer");

    let result = longest_with_announcement(&s1, &s2, "Here's a message!");
    println!("Longest: {result}");
}
