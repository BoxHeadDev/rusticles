// 🦀 Rustlings Challenge: Preventing Cycles with Weak<T>
//
// We're building a tree node that has:
// - strong Rc<RefCell<T>> references to its children (owns them)
// - weak parent links to avoid cycles (does NOT own the parent)
//
// Your tasks:
// 1. Add a weak parent pointer using Weak<T>.
// 2. Prevent memory leaks by ensuring children don't own their parents.
// 3. Use `upgrade()` to read from the weak parent reference.
//
// HINTS:
// - Rc::downgrade creates a Weak<T>
// - Weak::upgrade returns Option<Rc<T>>

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // ✅ Weak parent avoids cycle
    children: RefCell<Vec<Rc<Node>>>, // ✅ Strong references to owned children
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // ✅ Start with empty Weak
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), // ✅ Owns leaf
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // ✅ Non-owning back-reference

    // ✅ upgrade() returns Some(Rc<Node>) if branch still exists
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
