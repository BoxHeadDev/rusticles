// 🦀 Rustlings Challenge: Shared-State with Mutex in Single Thread
//
// This challenge introduces `Mutex<T>` in a single-threaded context.
// You'll use `.lock().unwrap()` to safely access and mutate the inner value.
//
// Tasks:
// 1. Use `.lock()` to acquire a mutable reference to the inner `i32`.
// 2. Update the value from 5 to 6.
// 3. Print the final value of the `Mutex<i32>`.
//
// HINTS:
// - The `.lock()` method returns a `MutexGuard`, which acts like a `&mut T`.
// - The lock is automatically released when the guard goes out of scope.

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut data = m.lock().unwrap(); // ✅ Acquire lock
        *data = 6; // ✅ Mutate value through MutexGuard
    } // ✅ Lock is released here when `data` goes out of scope

    println!("m = {m:?}"); // Output: m = Mutex { data: 6 }
}
