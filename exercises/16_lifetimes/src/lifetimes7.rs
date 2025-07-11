// 🦀 Rustlings Challenge: Generic Lifetimes in Functions
//
// You're writing a function that returns the longer of two string slices.
//
// This function must return a reference to one of its parameters — but Rust can't
// tell which one at compile time without a lifetime annotation.
//
// Your task is to fix the function signature by adding a generic lifetime parameter.
//
// ⚠️ Do NOT use `'static` — that's not the point of this exercise!
// ✅ Use a named lifetime that ties the input references to the output.
//
// HINT: You'll need to annotate all the references with the same lifetime.

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
