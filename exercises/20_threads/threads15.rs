// 🦀 Rustlings Challenge: Send and Rc<T> vs Arc<T>
//
// The `Send` trait indicates that a value's ownership can be transferred between threads.
// This challenge demonstrates why `Rc<T>` cannot be used in multithreaded scenarios.
//
// Tasks:
// 1. Try to spawn a thread using an Rc<T> — it won't compile!
// 2. Replace Rc<T> with Arc<T> to make it compile safely.
// 3. Observe that Arc<T> implements Send while Rc<T> does not.
//
// HINT: You only need `std::rc::Rc` or `std::sync::Arc`.
// HINT: `Arc<T>` stands for "Atomic Reference Counted".
//
// Expected output:
//     Hello from the thread: 42

use std::rc::Rc;
// use std::sync::Arc;
use std::thread;

fn main() {
    let data = Rc::new(42);

    let data_clone = Rc::clone(&data);
    let handle = thread::spawn(move || {
        // This line will fail to compile with Rc<T>:
        println!("Hello from the thread: {}", data_clone);
    });

    handle.join().unwrap();
}
