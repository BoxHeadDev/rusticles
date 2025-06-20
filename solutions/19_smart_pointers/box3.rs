// 🦀 Rustlings Challenge: Recursive List
//
// Recursive types require indirection so Rust knows how much space to allocate.
// Your task is to make the recursive enum `List` compile.
//
// HINT:
// - Use `Box<List>` in the `Cons` variant
// - You can nest `Box::new(...)` calls to build the full list

enum List {
    Cons(i32, Box<List>), // ✅ Added indirection with Box<List>
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // ✅ Each recursive step is boxed to allow finite type size
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
