// 🦀 Rustlings Challenge: Basic Thread Spawning
//
// You're learning to run multiple threads in Rust using `std::thread::spawn`.
// This example spawns a thread to print messages concurrently with the main thread.
//
// Tasks:
// 1. Run the code and observe if both threads print.
// 2. Fix the program so the spawned thread gets a chance to finish before the main thread exits.
// 3. Hint: Capture the JoinHandle and call `.join()`.
//
// EXPECTED (approximate) OUTPUT:
//     hi number 1 from the main thread!
//     hi number 1 from the spawned thread!
//     ...
//     hi number 9 from the spawned thread!

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // 🛑 The spawned thread may not finish before the main thread exits!
    // ✅ TODO: Use `.join()` on the thread handle to wait for it to finish.
}
