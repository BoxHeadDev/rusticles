// 🦀 Rustlings Challenge: Composing Streams
//
// This challenge simulates receiving a stream of real-time messages,
// some of which arrive too late and exceed a timeout threshold.
//
// - Complete the `get_messages()` function to:
//     - Spawn a task that sends 10 messages, one at a time
//     - Apply a 100ms delay to even messages, and a 300ms delay to odd ones
// - Apply a 200ms timeout to the stream
// - Pin the stream to allow polling
//
// EXPECTED OUTPUT (interleaved):
//   Message: 'a'
//   Problem: Elapsed(())
//   Message: 'c'
//   Message: 'e'
//   Problem: Elapsed(())
//   ...

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        // ✅ Timeout each message if it takes > 200ms
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

// TODO: Implement get_messages to return a Stream of delayed messages
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    // ✅ Spawn a background task to emit delayed messages
    // Hint: use index to switch delay between 100ms and 300ms
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.iter().enumerate() {
            let delay = if index % 2 == 0 { 100 } else { 300 };
            // TODO: Wait before sending each message
            // TODO: Send the formatted message through the channel
        }
    });

    ReceiverStream::new(rx)
}
