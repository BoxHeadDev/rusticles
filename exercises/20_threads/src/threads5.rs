// 🦀 Rustlings Challenge: Waiting for Threads with JoinHandles
//
// This challenge demonstrates how to wait for threads to finish using `.join()`.
//
// Tasks:
// 1. Move the `.join()` call so that output from both threads interleaves.
// 2. Observe how the order of `.join()` affects concurrency.
//
// HINT:
// - If `.join()` is called too early, the spawned thread will run *before* the main thread.
// - If `.join()` is called too late (or not at all), the spawned thread may not finish!
//
// Uncomment the `.join()` call in the right place to fix the behavior.

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // ✅ Interleave execution: don't wait yet!

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // TODO: Join the thread *after* the main thread finishes its loop
    // handle.join().unwrap();
}
