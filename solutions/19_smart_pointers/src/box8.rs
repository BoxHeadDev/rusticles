// 🦀 Rustlings Challenge: DerefMut
//
// Just like `Deref`, `DerefMut` lets you customize what happens when using `*`,
// but for **mutable** dereferencing (`*` on `&mut T`).
//
// This example illustrates all 3 coercion rules:
//
// - &T     -> &U     (when T: Deref<Target=U>)
// - &mut T -> &mut U (when T: DerefMut<Target=U>)
// - &mut T -> &U     (allowed)
// - &T     -> &mut U (❌ not allowed)

use std::ops::{Deref, DerefMut};

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

// ✅ Implement DerefMut to allow mutable dereferencing
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn shout(name: &mut String) {
    name.make_ascii_uppercase();
    println!("SHOUTING: {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    hello(&m); // ✅ &MyBox<String> -> &String -> &str

    let mut m2 = MyBox::new(String::from("loud rust"));
    shout(&mut m2); // ✅ &mut MyBox<String> -> &mut String

    hello(&m2); // ✅ &mut MyBox<String> -> &MyBox<String> -> &String -> &str
}
