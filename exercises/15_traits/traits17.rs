// This exercise introduces *blanket implementations*.
//
// A blanket implementation applies a trait to any type that satisfies the required trait bounds.
// The Rust standard library does this with `ToString` for any type that implements `Display`.
//
// Your task: Implement the trait `MyToString` for any type that implements `Display`.
//
// After implementing, the `to_my_string` method should work for types like `i32` and `String`.

use std::fmt::Display;

trait MyToString {
    fn to_my_string(&self) -> String;
}

// TODO: Implement MyToString for any T that implements Display
// using a blanket implementation.
impl /* your code here */ {
    fn to_my_string(&self) -> String {
        format!("{}", self)
    }
}

fn main() {
    let num = 42;
    let word = String::from("Rust");

    // These should work once the blanket implementation is complete.
    println!("{}", num.to_my_string());
    println!("{}", word.to_my_string());
}

