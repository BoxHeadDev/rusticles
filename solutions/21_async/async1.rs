// 🦀 Rustlings Challenge: Your First Async Function
//
// This challenge introduces writing and calling an async function in Rust.
//
// Tasks:
// 1. Complete the async function `page_title` that fetches a page and extracts the <title> tag.
// 2. In `main`, call the function using `.await`.
// 3. Print the result using `println!`, handling the Option properly.
//
// HINTS:
// - `trpl::get(url).await.text().await` gives you the response body.
// - Use `Html::parse(&text).select_first("title")` to get the title element.
// - `.map(|e| e.inner_html())` turns an Option<Element> into Option<String>.
// - `#[tokio::main]` lets you use `.await` inside `main`.
//
// Expected output (format may vary):
//     Title: The Rust Programming Language

use trpl::{Html, get};

async fn page_title(url: &str) -> Option<String> {
    let text = get(url).await.text().await;
    Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html())
}

#[tokio::main]
async fn main() {
    let url = "https://www.rust-lang.org";

    let title = page_title(url).await;

    match title {
        Some(t) => println!("Title: {}", t),
        None => println!("No title found."),
    }
}
