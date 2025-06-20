// 🦀 Rustlings Challenge: Multiple Producers, One Receiver
//
// This challenge shows how to use `mpsc::channel()` with multiple producers.
// You will clone the transmitter and spawn two threads that each send multiple messages.
//
// Tasks:
// 1. Clone the `Sender` to allow a second producer.
// 2. Spawn two threads, each sending a different set of messages.
// 3. Print all received messages in the main thread as they arrive.
//
// EXPECTED OUTPUT (order may vary due to thread scheduling):
// Got: hi
// Got: more
// Got: from
// Got: messages
// Got: for
// Got: the
// Got: thread
// Got: you

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // ✅ Clone the sender for the first producer

    // ✅ First producer thread
    thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // ✅ Second producer thread
    thread::spawn(move || {
        let msgs = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}"); // ✅ Output messages as they arrive
    }
}
