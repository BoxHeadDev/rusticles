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

use std::sync::Mutex;
use std::thread;
// ✅ TODO: Import Arc from std::sync
use std::sync::Arc;

fn main() {
    // ✅ TODO: Wrap the Mutex in an Arc
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        // ✅ TODO: Clone the Arc before moving into the thread
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // ✅ TODO: Lock the counter again to read the final value
    println!("Result: {}", *counter.lock().unwrap());
}
