// 🦀 Rustlings Challenge: Message Passing Between Threads
//
// In this exercise, you'll use a channel to send data between threads.
// One thread will send a message, and the main thread will receive and print it.
//
// Tasks:
// 1. Use `mpsc::channel()` to create a sender and receiver.
// 2. Use `thread::spawn()` to move the sender and send a `String`.
// 3. In the main thread, use `.recv()` to receive and print the message.
//
// HINTS:
// - `recv()` blocks until a message is available.
// - The channel must use `move` so the spawned thread owns the sender.
//
// EXPECTED OUTPUT:
// Got: hi

use std::thread;
// ✅ TODO: Import `mpsc` from `std::sync`
use std::sync::mpsc;

fn main() {
    // ✅ TODO: Create a channel (tx, rx)
    let (tx, rx) = mpsc::channel();

    // ✅ TODO: Spawn a thread and move `tx` into the closure
    thread::spawn(/* move */ || {
        let msg = String::from("hi");
        // ✅ TODO: Send the message using tx.send()
        // tx.send(msg).unwrap();
    });

    // ✅ TODO: Receive the message using rx.recv()
    // let received = rx.recv().unwrap();
    // println!("Got: {received}");
}
