// 🦀 Rustlings Challenge: Sync and Thread-Safe Shared References
//
// This challenge demonstrates the `Sync` trait, which lets you share *references* across threads.
//
// Tasks:
// 1. Try to send an `&RefCell<i32>` into a thread — it won't compile!
// 2. Replace the `RefCell<i32>` with a `Mutex<i32>` wrapped in `Arc<T>`.
// 3. Share an *immutable reference* to the Mutex<T> across threads, and mutate it safely.
//
// Expected output (order may vary):
//     Final result: 100

use std::cell::RefCell;
// use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let value = RefCell::new(0);
    // let value = Arc::new(Mutex::new(0));

    let handle = thread::spawn(move || {
        // ❌ This line will not compile: RefCell<T> is not Sync
        *value.borrow_mut() += 100;
    });

    handle.join().unwrap();

    println!("Final result: {}", value.borrow());
}
