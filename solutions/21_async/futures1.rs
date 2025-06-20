// 🦀 Rustlings Challenge: Manual Future Implementation
//
// In this exercise, you will implement the Future trait for a custom struct called `DelayCounter`.
// The future will:
// - Return `Poll::Pending` the first two times it's polled.
// - Return `Poll::Ready(String)` the third time.
//
// This simulates an async operation that takes time to complete.
// You must use Pin and Context properly to implement `poll`.
//
// Expected Output:
//     >> Polling future...
//     >> Not ready yet.
//     >> Polling future...
//     >> Not ready yet.
//     >> Polling future...
//     ✅ Ready: Done waiting!

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct DelayCounter {
    count: u8,
}

impl Future for DelayCounter {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut(); // Safe because we don't move inner fields
        this.count += 1;

        if this.count < 3 {
            Poll::Pending
        } else {
            Poll::Ready("Done waiting!".to_string())
        }
    }
}

fn main() {
    use futures::task::noop_waker;
    use std::task::Context;

    let mut counter = DelayCounter { count: 0 };
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut future = Pin::new(&mut counter);

    for _ in 0..3 {
        println!(">> Polling future...");
        match Future::poll(future.as_mut(), &mut cx) {
            Poll::Pending => println!(">> Not ready yet."),
            Poll::Ready(msg) => println!("✅ Ready: {msg}"),
        }
    }
}
