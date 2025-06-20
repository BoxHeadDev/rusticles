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
//
// TODOs:
// 1. Implement `Deref` and `DerefMut` for `MyBox<T>`
// 2. Uncomment the mutable function calls — they should compile!

use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// ✅ Implement Deref
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Implement DerefMut
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
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

    // ✅ Deref coercion from &MyBox<String> to &String to &str
    hello(&m);

    let mut m2 = MyBox::new(String::from("loud rust"));

    // TODO: Make this compile by implementing DerefMut
    // ✅ DerefMut allows &mut MyBox<T> to become &mut T
    // shout(&mut m2);

    // ✅ Also allowed: coercing &mut T to &T (but not the other way around)
    hello(&m2); // OK: &mut MyBox<T> -> &MyBox<T> -> &T -> &str

    // ❌ This would not compile (uncomment to test the error)
    // let m3 = MyBox::new(String::from("nope"));
    // shout(&m3); // &MyBox<String> can't coerce to &mut String
}
