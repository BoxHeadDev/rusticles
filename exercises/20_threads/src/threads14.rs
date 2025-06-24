// 🦀 Rustlings Challenge: Arc<T> + Mutex<T> for Thread-Safe Shared State
//
// Fix the following code so that 10 threads increment a shared counter.
// You'll need to replace Rc with Arc, and ensure Mutex usage is safe across threads.
//
// EXPECTED OUTPUT:
// Result: 10

use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
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
