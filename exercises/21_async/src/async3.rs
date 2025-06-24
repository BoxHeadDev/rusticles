// 🦀 Rustlings Challenge: Racing Two Async Futures
//
// This challenge demonstrates racing two concurrent futures using `trpl::race`.
//
// Tasks:
// 1. Modify `page_title` to return both the URL and the Option<String> title.
// 2. Use `trpl::race` to await whichever of the two page_title futures finishes first.
// 3. Use `match` to destructure the `trpl::Either` result.
// 4. Print out the URL that returned first, and its <title> if available.
//
// HINTS:
// - Use `Html::parse(&text).select_first("title")`.
// - Use `Either::Left(val)` and `Either::Right(val)`.
// - Use `.await` on both `trpl::get(...)` and `.text()`.

use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url1 = &args[1];
        let url2 = &args[2];

        // TODO: Call page_title on both URLs (but don't .await yet)
        let fut1 = page_title(url1);
        let fut2 = page_title(url2);

        // TODO: Race them and match on the result
        let (url, title) = match trpl::race(fut1, fut2).await {
            Either::Left(val) => val,
            Either::Right(val) => val,
        };

        println!("{url} returned first");
        match title {
            Some(t) => println!("Its page title is: '{t}'"),
            None => println!("Its title could not be parsed."),
        }
    });
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    // TODO: Fetch the page and return (url, title)
    todo!()
}
