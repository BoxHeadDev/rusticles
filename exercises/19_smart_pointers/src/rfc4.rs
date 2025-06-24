// 🦀 Rustlings Challenge: Shared Mutability with Rc<RefCell<T>>
//
// This challenge builds a recursive list using `Rc<RefCell<T>>` to allow shared, mutable data.
//
// Your task:
// 1. Replace the list's `i32` with `Rc<RefCell<i32>>`.
// 2. Modify the shared value through one owner.
// 3. Observe how it changes in all list instances sharing it.
//
// Expected output:
// a after = Cons(RefCell { value: 15 }, Nil)
// b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
// c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(/* TODO */ i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let shared_value = Rc::new(RefCell::new(5)); // ✅ Start with 5

    // TODO: Make `a` point to a list with the shared value
    let a = Rc::new(Cons(/* ??? */, Rc::new(Nil)));

    // TODO: Make `b` and `c` refer to `a` but have their own heads
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // ✅ Mutate shared value through one reference
    *shared_value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

