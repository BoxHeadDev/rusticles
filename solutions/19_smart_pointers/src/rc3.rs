// 🦀 Rustlings Challenge: Rc<T> Reference Counting
//
// `Rc<T>` is a smart pointer that enables multiple ownership by keeping a reference count.
// This challenge will help you observe how the reference count changes.
//
// Your tasks:
// 1. Replace `Box` with `Rc` in the List definition.
// 2. Use `Rc::clone()` to share the list.
// 3. Print the reference count after creating `a`, `b`, and `c`.
// 4. Notice how the count decreases when `c` goes out of scope.
//
// Use `Rc::strong_count(&a)` to check the count.

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    } // c goes out of scope here

    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}
