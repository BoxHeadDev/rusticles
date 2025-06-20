// 🦀 Rustlings Challenge: Moving Data into Threads
//
// Spawning threads often requires `move` closures to transfer data
// safely from the main thread into the new thread's environment.
//
// Tasks:
// 1. Fix the compiler error by moving `v` into the spawned thread.
// 2. Understand why `drop(v)` is no longer valid after the move.
//
// HINTS:
// - Use the `move` keyword to move ownership of `v`.
// - After moving `v`, it is no longer available in the main thread.
//
// Uncomment each section below step-by-step to test your understanding.

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // ❌ This closure tries to borrow `v`, but `v` is not guaranteed to live long enough.
    let handle = thread::spawn(/* TODO: add `move` here */ || {
        println!("Here's a vector: {v:?}");
    });

    // ❌ This causes a problem if `v` has been moved to the thread.
    // drop(v); // <- Uncomment this line to see what happens

    handle.join().unwrap();
}
