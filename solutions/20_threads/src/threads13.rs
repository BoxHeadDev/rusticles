// 🦀 Rustlings Challenge: Rc<T> and Thread Safety
//
// In this challenge, you'll encounter a compile error when trying to share an Rc<Mutex<T>> across threads.
//
// Tasks:
// 1. Try compiling this example to see the `Rc<T>` thread-safety error.
// 2. Replace `Rc<T>` with `Arc<T>` to fix the issue.
// 3. Ensure all 10 threads safely increment the counter.
//
// HINTS:
// - `Rc<T>` is NOT thread-safe; `Arc<T>` (atomic reference count) is.
// - You'll need to import `Arc` from `std::sync`.
//
// EXPECTED OUTPUT:
// Result: 10

use std::sync::{Arc, Mutex}; // ✅ Import Arc for thread-safe reference counting
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // ✅ Use Arc instead of Rc
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // ✅ Clone Arc for shared ownership
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // ✅ Safely access shared state
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // ✅ Wait for all threads
    }

    println!("Result: {}", *counter.lock().unwrap()); // ✅ Final output: 10
}
