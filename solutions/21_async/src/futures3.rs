// 🦀 Rustlings Challenge: Manual Stream Implementation
//
// Your goal is to manually implement the Stream trait for a struct `CounterStream`
// that asynchronously yields values 1, 2, 3 (and then ends).
//
// Stream is similar to an async iterator — the main method is `poll_next`,
// which returns `Poll::Pending`, or `Poll::Ready(Some(item))`, or `Poll::Ready(None)`

use std::pin::Pin;
use std::task::{Context, Poll};

// 🧪 This is the same trait used in the `futures` crate
trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

struct CounterStream {
    current: u8,
}

impl Stream for CounterStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut(); // Safe: no move occurs
        this.current += 1;

        if this.current <= 3 {
            Poll::Ready(Some(this.current))
        } else {
            Poll::Ready(None)
        }
    }
}

fn main() {
    use futures::task::noop_waker;
    use std::task::Context;

    let mut stream = CounterStream { current: 0 };
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut pin_stream = Pin::new(&mut stream);

    for _ in 0..5 {
        match pin_stream.as_mut().poll_next(&mut cx) {
            Poll::Pending => println!("⏳ Not ready yet"),
            Poll::Ready(Some(n)) => println!("✅ Got value: {n}"),
            Poll::Ready(None) => {
                println!("🚫 Stream is done");
                break;
            }
        }
    }
}
