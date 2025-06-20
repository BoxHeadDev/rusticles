// 🦀 Rustlings Challenge: Shared Ownership with Rc<T>
//
// Update the definition of `List` so it supports multiple owners of the same list.
// Currently, the list uses `Box<T>`, which only allows a single owner.
//
// Your tasks:
// 1. Change the `Box<T>` usage to `Rc<T>`.
// 2. Use `Rc::clone` to share the inner list between `b` and `c`.
//
// HINTS:
// - You’ll need to import Rc from std::rc.
// - Rc::clone increases the reference count (it doesn’t clone the data).
// - You *cannot* use the same `a` twice with Box<T> — try it and see the compiler error!
//
// Expected behavior: both `b` and `c` point to the same sublist (shared through Rc).

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    let b = Cons(3, Rc::clone(&a)); // ✅ Shared ownership of `a`
    let c = Cons(4, Rc::clone(&a)); // ✅ Still valid because Rc was cloned

    // ✅ Rc::clone only increments ref count, no deep copy
    println!("Shared list created successfully!");
}
