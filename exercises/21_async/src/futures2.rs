// 🦀 Rustlings Challenge: Pin and Unpin
//
// In this challenge, you’ll simulate pinning a self-referential future
// and working with types that *do* or *do not* implement `Unpin`.
//
// Your tasks:
// - Use `Pin<Box<T>>` to pin a future that doesn't implement `Unpin`.
// - Demonstrate that a type like `String` *can* be moved even when pinned.
// - Understand the compiler error you get when trying to use `.await` on a boxed future that is not `Unpin`.
//
// Hint: Use `Box::pin(...)` and `Pin::new(...)`
// Hint: Try `.is_unpin()` to see if a type implements `Unpin`

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

// ✅ A simple future that is !Unpin
struct NotUnpin {
    state: u8,
    data: String,
}

// Explicitly declare this type is not Unpin.
impl !Unpin for NotUnpin {}

impl Future for NotUnpin {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        this.state += 1;

        if this.state >= 2 {
            Poll::Ready("done")
        } else {
            Poll::Pending
        }
    }
}

fn main() {
    use futures::task::noop_waker;
    use std::task::Context;

    // ✅ Try to pin and poll `NotUnpin`
    let mut fut = Box::pin(NotUnpin {
        state: 0,
        data: String::from("hello"),
    });

    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    println!("Polling pinned future...");
    let _ = fut.as_mut().poll(&mut cx);

    // ✅ Show that String is Unpin
    let s = String::from("can move");
    let pinned = Pin::new(&s);
    println!("Is String Unpin? {}", Pin::into_inner(pinned).is_empty());

    // 🚧 BONUS: Try using `join_all` on a vector of Box::pin'd futures here
    // let futs = vec![Box::pin(NotUnpin { ... }), ...];
    // What error do you get, and how would you fix it?
}
