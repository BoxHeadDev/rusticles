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

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // ✅ Create the channel

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap(); // ✅ Send message from spawned thread
    });

    let received = rx.recv().unwrap(); // ✅ Block and receive message in main thread
    println!("Got: {received}"); // Output: Got: hi
}
