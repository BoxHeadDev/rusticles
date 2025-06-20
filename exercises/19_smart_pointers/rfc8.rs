// 🦀 Rustlings Challenge: Tree with Parent References Using Weak<T>
//
// In this exercise, you'll build a tree node with both child and parent references.
// Children own their parents through `Rc`, but parents must NOT own their children.
//
// Your tasks:
// 1. Use Rc for children (strong ownership).
// 2. Use Weak for parent (non-owning back reference).
// 3. Link leaf -> parent using Rc::downgrade().
// 4. Use .upgrade() to verify that the parent is accessible.
//
// Expected output:
// leaf parent = None
// leaf parent = Some(Node { value: 5, ... })

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // ✅ Weak reference to prevent cycles
    children: RefCell<Vec<Rc<Node>>>, // ✅ Strongly owned children
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // ✅ No parent initially
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), // ✅ branch owns leaf
    });

    // TODO: Set leaf's parent to point to branch using downgrade
    // *leaf.parent.borrow_mut() = ...;

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
