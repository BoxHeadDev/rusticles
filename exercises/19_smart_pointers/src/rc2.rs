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

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

    // 🛑 This line works because `a` is moved here.
    let b = Cons(3, Box::new(a));

    // 🛑 ERROR: `a` has already been moved. This line will not compile!
    let c = Cons(4, Box::new(a));

    println!("If you're seeing this, your code compiled — but it shouldn't yet!");
}
