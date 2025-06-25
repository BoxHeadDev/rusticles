// 🦀 Rustlings Challenge: Yielding Control to the Runtime
//
// You’re simulating two long-running futures. The goal is to break up blocking work
// so the runtime can interleave execution between them.
//
// - Implement the `slow` function using `std::thread::sleep`.
// - Use `trpl::yield_now().await` between calls to `slow` to give the runtime a chance
//   to schedule the other future.
// - You will race the two futures and see interleaved output if you yield properly.
//
// EXPECTED OUTPUT (interleaved, order may vary slightly):
//     'a' started.
//     'a' ran for 30ms
//     'b' started.
//     'b' ran for 75ms
//     'a' ran for 10ms
//     'b' ran for 10ms
//     'a' ran for 20ms
//     'b' ran for 15ms
//     'a' finished.

use std::thread;
use std::time::Duration;

mod trpl {
    use async_std::task;
    use futures::{future::Either, future::FutureExt};

    pub async fn sleep(duration: std::time::Duration) {
        task::sleep(duration).await
    }

    pub async fn yield_now() {
        task::yield_now().await
    }

    pub async fn race<F1, F2>(f1: F1, f2: F2) -> Either<F1::Output, F2::Output>
    where
        F1: std::future::Future + Unpin,
        F2: std::future::Future + Unpin,
    {
        futures::future::select(f1, f2).await
    }
}

// ✅ Blocking function simulating long computation
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

#[async_std::main]
async fn main() {
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        trpl::yield_now().await; // ✅ Give control to runtime
        slow("a", 10);
        trpl::yield_now().await; // ✅ Yield again
        slow("a", 20);
        trpl::yield_now().await; // ✅ And again
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        trpl::yield_now().await; // ✅ Interleave with future `a`
        slow("b", 10);
        trpl::yield_now().await;
        slow("b", 15);
        trpl::yield_now().await;
        println!("'b' finished.");
    };

    trpl::race(a, b).await;
}
