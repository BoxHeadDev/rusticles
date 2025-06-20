// 🦀 Rustlings Challenge: Define a Smart Pointer with Deref
//
// This version of `MyBox<T>` implements the `Deref` trait,
// so you can now use `*mybox` to access the inner value like a regular reference.

use std::ops::Deref;

// ✅ Define the smart pointer type
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// ✅ Implement Deref so `*MyBox<T>` returns a reference to `T`
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 42;
    let y = MyBox::new(x);

    assert_eq!(42, x);
    assert_eq!(42, *y); // ✅ Now works because Deref is implemented
}
