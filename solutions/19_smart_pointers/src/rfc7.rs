// 🦀 Rustlings Challenge: Tree Structure with Rc and RefCell
//
// In this challenge, you'll build a tree structure where a `Node` can have multiple children.
// Each child node is reference counted (`Rc`) so multiple parts of the program can access it.
// We also use `RefCell` to allow interior mutability.
//
// Your tasks:
// 1. Complete the `Node` struct with a `value: i32` and a list of `Rc<Node>` children.
// 2. Create a `leaf` node with no children.
// 3. Create a `branch` node with `leaf` as its child using `Rc::clone`.
// 4. Print both nodes to verify the relationship.
//
// Expected output:
// leaf: Node { value: 3, children: [] }
// branch: Node { value: 5, children: [Node { value: 3, children: [] }] }

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>, // ✅ Allows shared ownership and interior mutability
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]), // ✅ Clone Rc to share ownership
    });

    println!("leaf: {:?}", leaf);
    println!("branch: {:?}", branch);
}
