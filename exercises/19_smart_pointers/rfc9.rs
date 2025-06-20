// 🦀 Rustlings Challenge: Rc<T> and Weak<T> Reference Counting
//
// In this challenge, you'll explore how strong and weak counts behave
// when using `Rc<T>` and `Weak<T>` together with interior mutability.
//
// Tasks:
// 1. Use `Rc::downgrade` to create a weak reference from `leaf.parent` to `branch`.
// 2. Print `strong_count` and `weak_count` to observe how they change over time.
// 3. After the scope ends, check that the weak pointer was dropped automatically.
//
// Expected behavior:
// - leaf starts with strong = 1, weak = 0
// - branch adds another strong to leaf and gets a weak from leaf
// - after branch goes out of scope, parent is dropped, and weak can't be upgraded

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
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]), // ✅ strong count on leaf increases
        });

        // ✅ Set leaf's parent to a weak reference to branch
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // ✅ branch is dropped here, its Node is dropped too
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

