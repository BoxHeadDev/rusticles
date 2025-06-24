// 🦀 Rustlings Challenge: Merging Streams
//
// In this challenge, you'll:
// - Merge two `Stream`s: one of messages, one of interval ticks.
// - Apply `.timeout()` and `.throttle()` to avoid overloading.
// - Use `.take()` to stop after N total items.
// - Handle channel `send` errors gracefully.
//
// EXPECTED OUTPUT (interleaved):
//   Interval: 1
//   Message: 'a'
//   Interval: 2
//   Interval: 3
//   Problem: Elapsed(())
//   ...

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));

        // ✅ Merge both streams into a single stream of Result<String, Elapsed>
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        // ✅ Print out values or errors
        while let Some(item) = stream.next().await {
            match item {
                Ok(value) => println!("{value}"),
                Err(e) => eprintln!("Problem: {e:?}"),
            }
        }
    });
}

// ✅ Implement this to send delayed messages
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let sleep_ms = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(sleep_ms)).await;

            // TODO: Handle send error by breaking the loop
            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

// ✅ Implement this to send interval counts
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            // TODO: Handle send error by breaking the loop
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
