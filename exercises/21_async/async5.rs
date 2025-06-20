// 🦀 Rustlings Challenge: Multiple Async Tasks with Channels
//
// This challenge uses `trpl::channel` to send messages from two producers
// to one receiver task. It demonstrates:
// - async message passing
// - async move blocks
// - concurrent execution with join3
//
// Tasks:
// 1. Clone the sender and create two async move blocks that send different messages.
// 2. Use `trpl::sleep` with different delays in each sender.
// 3. Use a receiver loop with `while let Some(...) = rx.recv().await` to print each message.
// 4. Run all three async blocks using `trpl::join3`.
//
// EXPECTED OUTPUT (timing may vary):
// received 'hi'
// received 'more'
// received 'from'
// received 'the'
// received 'messages'
// received 'future'
// received 'for'
// received 'you'

use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // ✅ Clone the sender for the first task
        let tx1 = tx.clone();

        // TODO: Send "hi", "from", "the", "future" with 500ms delay
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // TODO: Send "more", "messages", "for", "you" with 1500ms delay
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // TODO: Receive and print messages as they arrive
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // ✅ Run all futures concurrently
        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}
