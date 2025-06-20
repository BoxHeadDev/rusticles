// 🦀 Rustlings Challenge: Stream From Iterator
//
// Streams are like async iterators: they yield values over time using `await`.
// Your task is to:
//
// - Convert an iterator to a stream using `trpl::stream_from_iter`.
// - Add `use trpl::StreamExt;` to bring async stream methods into scope.
// - Use `.filter(...)` to only keep values that are divisible by 3 or 5.
// - Use `.next().await` in a loop to consume the stream.
//
// Expected output (filtered values only):
//     The value was: 6
//     The value was: 10
//     ...
//     (only values divisible by 3 or 5)

mod trpl {
    use async_std::stream;
    use async_std::task;
    pub use futures::stream::{Stream, StreamExt}; // ✅ Needed for `.next()` etc.

    pub fn run<F: std::future::Future<Output = ()>>(fut: F) {
        task::block_on(fut)
    }

    pub fn stream_from_iter<I>(iter: I) -> impl Stream<Item = I::Item>
    where
        I: IntoIterator,
    {
        stream::from_iter(iter)
    }
}

fn main() {
    // TODO: Bring StreamExt methods into scope.
    // Hint: use trpl::StreamExt;

    trpl::run(async {
        let values = 1..=20;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        // TODO: Filter values that are divisible by 3 or 5
        let mut filtered = stream; // <-- replace this with `.filter(...)`

        // TODO: Use `while let Some(value) = ...` to consume the stream with `.next().await`
        // and print out each value
    });
}
