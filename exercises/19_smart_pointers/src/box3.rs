// 🦀 Rustlings Challenge: Recursive List
//
// Recursive types require indirection so Rust knows how much space to allocate.
// Your task is to make the recursive enum `List` compile.
//
// HINT:
// - Use `Box<List>` in the `Cons` variant
// - You can nest `Box::new(...)` calls to build the full list

// TODO: Fix the enum definition so it compiles
enum List {
    Cons(i32, List), // ❌ recursive type without indirection!
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Create a list: 1 -> 2 -> 3 -> Nil
    let _list = Cons(1, Cons(2, Cons(3, Nil)));
    // ✅ After fixing the enum, wrap inner list values with Box::new(...)
}
