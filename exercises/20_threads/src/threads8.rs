// 🦀 Rustlings Challenge: Channels and Ownership Transference
//
// This challenge demonstrates how Rust prevents use-after-move bugs in multithreaded code.
//
// Tasks:
// 1. Try to use a variable (`val`) after it's sent via the channel.
// 2. Observe the compiler error and understand why it's correct.
// 3. Comment out the invalid usage and get the code to compile.
//
// HINT:
// - `tx.send(val)` moves the value out of `val`, which invalidates it.
// - You can't use `val` after it has been moved!
//
// EXPECTED OUTPUT:
// Got: hi

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // val is moved here

        // ❌ Uncommenting this line will cause a compile-time error
        // println!("val is {val}"); // ⚠️ Cannot use val after it’s moved!
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
