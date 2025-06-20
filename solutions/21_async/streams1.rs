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

use trpl::StreamExt; // ✅ Brings `.filter()` and `.next()` into scope

fn main() {
    trpl::run(async {
        let values = 1..=20;
        let iter = values.map(|n| n * 2); // doubles: 2, 4, 6, ..., 40
        let stream = trpl::stream_from_iter(iter);

        // ✅ Filter only values divisible by 3 or 5
        let mut filtered = stream.filter(|value| {
            let is_valid = *value % 3 == 0 || *value % 5 == 0;
            async move { is_valid }
        });

        // ✅ Consume the stream
        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}
