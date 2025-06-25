// 🦀 Rustlings Challenge: Deref Coercion
//
// When you pass `&MyBox<String>` to a function expecting `&str`,
// Rust automatically dereferences it with the `Deref` trait.
//
// Your task is to:
// - Implement `Deref` for `MyBox<T>`
// - Pass a `MyBox<String>` to a function that expects `&str`
// - Observe that you don’t need to manually write `&(*mybox)[..]`
//
// HINTS:
// - Use `std::ops::Deref`
// - Your `deref()` method should return `&T`

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    // ✅ Deref coercion: &MyBox<String> -> &String -> &str
    hello(&m);
}
