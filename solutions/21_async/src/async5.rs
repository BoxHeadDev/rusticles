// 🦀 Rustlings Challenge: Multiple Async Tasks with Channels
//
// This challenge uses `trpl::channel` to send messages from two producers
// to one receiver task. It demonstrates:
// - async message passing
// - async move blocks
// - concurrent execution with join3
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

        let tx1 = tx.clone();

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

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}
