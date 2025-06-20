// 🦀 Rustlings Challenge: Dynamically Joining Futures
//
// In this challenge, you'll simulate two "producers" sending messages
// over a channel to one "consumer" — and wait for all 3 tasks to complete.
//
// The twist? Instead of using `trpl::join3`, you'll collect the futures into a `Vec`
// and pass them to `trpl::join_all`.
//
// Tasks:
// 1. Wrap each future in `Box::pin()` and give them the same trait object type.
// 2. Annotate the Vec with an explicit type: `Vec<Pin<Box<dyn Future<Output = ()>>>>`.
// 3. Pass the Vec to `trpl::join_all(...)` and `.await` the result.
//
// Hint: Add `use std::pin::Pin; use std::future::Future;`
//
// Expected output (timing may vary):
// received 'hi'
// received 'more'
// received 'from'
// received 'messages'
// received 'the'
// received 'for'
// received 'future'
// received 'you'

use std::time::Duration;
// TODO: Add necessary imports

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx1_fut = async move {
            let vals = vec!["hi", "from", "the", "future"];
            for val in vals {
                tx1.send(val.to_string()).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut = async move {
            let vals = vec!["more", "messages", "for", "you"];
            for val in vals {
                tx.send(val.to_string()).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // TODO: Box and pin the futures so they can go in a Vec
        // let futures = vec![...];

        // TODO: Await them all with trpl::join_all
        // trpl::join_all(futures).await;
    });
}
