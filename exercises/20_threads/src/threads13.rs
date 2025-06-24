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

use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // ❌ Rc is not thread-safe
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
