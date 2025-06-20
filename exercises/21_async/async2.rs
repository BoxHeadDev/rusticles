// 🦀 Rustlings Challenge: Async Page Title from CLI
//
// The `async fn main()` syntax is not allowed in Rust — you need a runtime.
// Use `trpl::run` to run an `async` block instead.
//
// Tasks:
// 1. Write an async block passed to `trpl::run` in `main`.
// 2. Call `page_title(url).await` inside it.
// 3. Match on the result and print either the title or a fallback message.
//
// HINT:
// - Use `std::env::args().collect()` to get command-line arguments.
// - Use `{}` in println! to print string variables.
//
// Example usage:
// $ cargo run -- https://www.rust-lang.org
// The title for https://www.rust-lang.org was Rust Programming Language

use trpl::{Html, get};

async fn page_title(url: &str) -> Option<String> {
    let text = get(url).await.text().await;
    Html::parse(&text)
        .select_first("title")
        .map(|e| e.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // TODO: Use `trpl::run` to run an async block
    // Inside it:
    // - Get the first URL from args
    // - Call `page_title(url).await`
    // - Print either the title or "no title" message
}
