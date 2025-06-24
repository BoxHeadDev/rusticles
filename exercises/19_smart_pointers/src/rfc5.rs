// 🦀 Rustlings Challenge: Reference Cycles and Memory Leaks
//
// In this challenge, you’ll build two `List` nodes that reference each other,
// forming a cycle. Although this is memory safe, it creates a *memory leak*.
//
// Your tasks:
// 1. Build `List` using Rc<RefCell<...>> to allow shared, mutable links.
// 2. Connect `a` and `b` into a reference cycle.
// 3. Print reference counts to observe the leak.
// 4. (Optional) Uncomment the final `println!` to see the stack overflow.
//
// WARNING: The final `println!` will cause an infinite loop via debug printing.
// Leave it commented unless you're intentionally testing cycle behavior.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use crate::List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));

    // TODO: Create a cycle by modifying a's tail to point to b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // ✅ Mutates a -> b
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // ⚠️ Uncommenting the line below causes infinite recursion due to cycle
    // println!("a next item = {:?}", a.tail());
}
