// 🦀 Rustlings Challenge: Racing Futures
//
// In this exercise, you'll race two async blocks and observe their execution behavior.
// The `trpl::race` function returns as soon as one future completes.
//
// Your goal is to:
// - Add the correct durations for both `fast` and `slow` futures.
// - Understand that the order of `.race(f1, f2)` affects which one gets polled first.
//
// Use `Duration::from_millis(n)` to simulate delay.
//
// Expected output (fast should always win):
//     'fast' started.
//     'slow' started.
//     'fast' finished.
//
// (Slow will not finish because the race completes with the first future.)

use std::time::Duration;

mod trpl {
    use futures::{future::Either, future::FutureExt};
    use std::time::Duration;

    pub async fn sleep(duration: Duration) {
        async_std::task::sleep(duration).await
    }

    pub async fn race<F1, F2>(f1: F1, f2: F2) -> Either<F1::Output, F2::Output>
    where
        F1: std::future::Future + Unpin,
        F2: std::future::Future + Unpin,
    {
        futures::future::select(f1, f2).await
    }
}

#[async_std::main]
async fn main() {
    let slow = async {
        println!("'slow' started.");
        trpl::sleep(Duration::from_millis(100)).await; // ⏱️ Slower future
        println!("'slow' finished.");
    };

    let fast = async {
        println!("'fast' started.");
        trpl::sleep(Duration::from_millis(50)).await; // ✅ Faster future
        println!("'fast' finished.");
    };

    trpl::race(slow, fast).await; // ✅ Fast finishes, slow never completes

    println!("Race completed.");
}
