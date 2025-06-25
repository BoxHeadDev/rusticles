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
    Cons(Rc<RefCell<i32>>, Rc<List>), // ✅ Shared, mutable i32
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let shared_value = Rc::new(RefCell::new(5)); // ✅ Shared value starts at 5

    let a = Rc::new(Cons(Rc::clone(&shared_value), Rc::new(Nil))); // ✅ a owns the shared value

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)); // ✅ b points to a
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)); // ✅ c also points to a

    *shared_value.borrow_mut() += 10; // ✅ Mutate value from 5 to 15

    // ✅ All references now reflect the mutated value
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
