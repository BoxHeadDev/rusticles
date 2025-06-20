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

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });
}
