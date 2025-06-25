// 🦀 Rustlings Challenge: Async Timeout
//
// You're going to build a `timeout` utility that races a future against a timeout duration.
// If the future completes before the timer, return `Ok(value)`.
// If the timer finishes first, return `Err(duration)`.
//
// Tasks:
// - Implement the `timeout` function.
// - Use `trpl::race` to race the future against a timer built with `trpl::sleep`.
// - Ensure that the passed future gets polled first, as `race` is not fair.
//
// Expected output (actual result may vary based on duration used):
//   - Succeeded with 'I finished!'
//   - OR: Failed after X seconds

use std::future::Future;
use std::time::Duration;

mod trpl {
    use async_std::task;
    use futures::{future::Either as FutEither, future::FutureExt};

    pub async fn sleep(duration: std::time::Duration) {
        task::sleep(duration).await
    }

    pub async fn race<F1, F2>(f1: F1, f2: F2) -> FutEither<F1::Output, F2::Output>
    where
        F1: std::future::Future + Unpin,
        F2: std::future::Future + Unpin,
    {
        futures::future::select(f1, f2).await
    }

    pub enum Either<L, R> {
        Left(L),
        Right(R),
    }

    pub fn run<F: std::future::Future<Output = ()>>(fut: F) {
        task::block_on(fut)
    }
}

use trpl::Either;

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(msg) => println!("Succeeded with '{msg}'"),
            Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
        }
    });
}

// ✅ Implements timeout using race between future and sleep timer
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        futures::future::Either::Left(output) => Ok(output), // ✅ Future completed first
        futures::future::Either::Right(_) => Err(max_time),  // ✅ Timer expired first
    }
}
