// 🦀 Rustlings Challenge: Multiple Values Over a Channel
//
// This challenge demonstrates message passing by sending multiple values from a thread.
// The receiving thread (main) waits and prints messages as they arrive.
//
// Tasks:
// 1. Send a vector of strings one at a time over the channel.
// 2. Sleep 1 second between each send in the spawned thread.
// 3. Print each received message in the main thread.
//
// EXPECTED OUTPUT (approximate, with 1s pause between lines):
// Got: hi
// Got: from
// Got: the
// Got: thread

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for msg in messages {
            // ✅ TODO: Send the message through the channel
            // ✅ TODO: Sleep for 1 second between messages
            // tx.send(msg).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    // ✅ TODO: Receive messages as they arrive and print them
    // for received in rx {
    //     println!("Got: {received}");
    // }
}
