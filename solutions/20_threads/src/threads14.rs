// ✅ Fixed: Arc<T> used in place of Rc<T> for thread safety
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // ✅ Safe reference counting across threads
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // ✅ Mutex guards the value safely
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // ✅ Wait for all threads to finish
    }

    println!("Result: {}", *counter.lock().unwrap()); // ✅ Result: 10
}
