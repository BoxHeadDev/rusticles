// 🦀 Rustlings Challenge: Dynamically Joining Futures
//
// See challenge instructions above.

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

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

        // ✅ Use Box::pin and Pin<Box<...>> to store different futures of same Output type
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(tx_fut), Box::pin(rx_fut)];

        trpl::join_all(futures).await;
    });
}
