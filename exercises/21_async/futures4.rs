// 🦀 Rustlings Challenge: Futures, Tasks, and Threads
//
// This exercise combines `std::thread::spawn` with async futures.
// You'll use a channel to send values from a thread to an async context.
//
// ✅ Goals:
// - Spawn a thread that sends values 1 through 5, once per second
// - Receive and print those values from async code using `.recv().await`
// - Use `trpl::channel()` and `trpl::run()` to coordinate the runtime
//
// EXPECTED OUTPUT (approximately 1 line per second):
//     Received: 1
//     Received: 2
//     ...
//     Received: 5

use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    // TODO: Spawn a thread that:
    // - Loops from 1 to 5
    // - Sends each number over the channel
    // - Sleeps 1 second between sends

    // TODO: Start the async runtime using trpl::run and receive messages
    // Use: while let Some(value) = rx.recv().await { ... }
}
