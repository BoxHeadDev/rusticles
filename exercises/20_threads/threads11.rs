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
        // ✅ TODO: Acquire the lock and mutate the inner value to 6
        // let mut data = ...;
        // *data = 6;
    }

    // ✅ Print the mutex to verify the update
    println!("m = {m:?}"); // Expected output: m = Mutex { data: 6 }
}
