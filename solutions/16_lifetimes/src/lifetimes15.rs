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

// ✅ Added lifetime `'a` and generic `T`, used `where` clause for clarity
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("a little bit longer");

    let result = longest_with_announcement(&s1, &s2, "Here's a message!");
    println!("Longest: {result}");
}
