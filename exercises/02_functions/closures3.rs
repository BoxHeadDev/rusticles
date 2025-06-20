// 🦀 Rustlings Challenge: Closure Capture Modes
//
// Closures can capture values from their environment in three ways:
// 1. Immutable borrow (like `&T`)
// 2. Mutable borrow (like `&mut T`)
// 3. Taking ownership (like `T`)
//
// Your task is to complete and fix the following examples demonstrating each kind of capture.
// Pay attention to how borrow checking and ownership rules apply.
//
// HINTS:
// - The first example only needs an immutable borrow.
// - The second example needs a mutable borrow.
// - The third example must use `move` to transfer ownership to a thread.

use std::thread;

fn main() {
    // 🧪 Part 1: Immutable Borrow
    let list1 = vec![1, 2, 3];
    println!("Before defining closure (immutable): {list1:?}");

    let print_list = || {
        println!("Inside closure (immutable): {list1:?}");
    };

    print_list();
    println!("After calling closure (immutable): {list1:?}");

    // 🧪 Part 2: Mutable Borrow
    let mut list2 = vec![10, 20, 30];
    println!("Before defining closure (mutable): {list2:?}");

    let mut push_value = || {
        // TODO: Add a value to `list2` inside this closure.
    };

    // println!("Before calling closure (mutable): {list2:?}"); // ⚠️ This line will cause a borrow error
    push_value();
    println!("After calling closure (mutable): {list2:?}");

    // 🧪 Part 3: Move Ownership into Thread
    let list3 = vec![100, 200, 300];
    println!("Before spawning thread: {list3:?}");

    // TODO: Use `move` to give ownership of `list3` to the thread
    let handle = thread::spawn(|| {
        println!("From thread: {list3:?}");
    });

    handle.join().unwrap();

    // println!("Back in main: {list3:?}"); // ⚠️ Can't use `list3` here after move
}
