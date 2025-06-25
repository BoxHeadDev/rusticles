// 🦀 Rustlings Challenge: Sharing a Mutex Across Threads
//
// This challenge demonstrates using `Arc<Mutex<T>>` to allow shared,
// mutable access to a value from multiple threads.
//
// Tasks:
// 1. Wrap the `Mutex<i32>` in an `Arc<T>` to enable multiple ownership.
// 2. Clone the `Arc<T>` before moving it into each thread.
// 3. Each thread should increment the counter by 1.
// 4. The final result should be 10.
//
// Hint: Arc = "Atomically Reference Counted", and it's required for sharing across threads.
//
// EXPECTED OUTPUT:
// Result: 10

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // ✅ Arc allows multiple owners

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // ✅ Clone Arc for each thread
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // ✅ Lock the mutex
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // ✅ Wait for all threads
    }

    println!("Result: {}", *counter.lock().unwrap()); // ✅ Final value = 10
}
