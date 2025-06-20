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

    thread::spawn(move || {
        for i in 1..=5 {
            if let Err(e) = tx.send(i) {
                eprintln!("Send error: {e}");
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(value) = rx.recv().await {
            println!("Received: {value}");
        }
    });
}
