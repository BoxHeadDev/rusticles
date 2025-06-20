// 🦀 Rustlings Challenge: Custom Deref
//
// In this challenge, you’ll implement a smart pointer called `MyBox<T>`
// that behaves like `Box<T>` by implementing the `Deref` trait.
//
// This lets you use `*my_box` like `*reference` to access the inner value.
//
// HINTS:
// - You'll need to implement the `Deref` trait from `std::ops`
// - The `deref` method should return a reference to the inner value (`&T`)

use std::ops::Deref;

struct MyBox<T>(T);

// ✅ Implement a constructor
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// ✅ Implement Deref for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // ✅ Return a reference to the inner value
    }
}

fn main() {
    let x = 10;
    let y = MyBox::new(x);

    assert_eq!(10, x);
    assert_eq!(10, *y); // ✅ `*y` calls Deref::deref and compares value
}
