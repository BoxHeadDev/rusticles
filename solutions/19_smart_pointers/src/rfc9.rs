// 🦀 Rustlings Challenge: Rc<T> and Weak<T> Reference Counting
//
// In this challenge, you'll explore how strong and weak counts behave
// when using `Rc<T>` and `Weak<T>` together with interior mutability.
//
// Tasks:
// 1. Use `Rc::downgrade` to create a weak reference from `leaf.parent` to `branch`.
// 2. Print `strong_count` and `weak_count` to observe how they change over time.
// 3. After the scope ends, check that the weak pointer was dropped automatically.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    ); // Output: strong = 1, weak = 0

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // ✅ Create weak parent link

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        ); // strong = 1, weak = 1

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        ); // strong = 2 (leaf and branch), weak = 0
    }

    // ✅ branch is dropped, weak pointer can't be upgraded
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // Output: None

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    ); // Output: strong = 1, weak = 0
}
