// 🦀 Rustlings Challenge: Closure Capture Modes
//
// Closures can capture values from their environment in three ways:
// 1. Immutable borrow (like `&T`)
// 2. Mutable borrow (like `&mut T`)
// 3. Taking ownership (like `T`)
//
// Your task is to complete and fix the following examples demonstrating each kind of capture.
// Pay attention to how borrow checking and ownership rules apply.

use std::thread;

fn main() {
    // ✅ Part 1: Immutable Borrow
    let list1 = vec![1, 2, 3];
    println!("Before defining closure (immutable): {list1:?}");

    let print_list = || {
        println!("Inside closure (immutable): {list1:?}"); // ✅ captures immutable reference
    };

    print_list();
    println!("After calling closure (immutable): {list1:?}");

    // ✅ Part 2: Mutable Borrow
    let mut list2 = vec![10, 20, 30];
    println!("Before defining closure (mutable): {list2:?}");

    let mut push_value = || {
        list2.push(40); // ✅ captures mutable reference
    };

    // println!("Before calling closure (mutable): {list2:?}"); // ❌ would conflict with mutable borrow
    push_value();
    println!("After calling closure (mutable): {list2:?}");

    // ✅ Part 3: Move Ownership into Thread
    let list3 = vec![100, 200, 300];
    println!("Before spawning thread: {list3:?}");

    let handle = thread::spawn(move || {
        println!("From thread: {list3:?}"); // ✅ takes ownership with `move`
    });

    handle.join().unwrap();

    // println!("Back in main: {list3:?}"); // ❌ `list3` has been moved into the thread
}
