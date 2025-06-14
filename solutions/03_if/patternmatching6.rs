// Solution: Borrow `opt` in the `match` statement to avoid moving ownership
fn main() {
    let opt: Option<String> = Some(String::from("Hello world"));

    // Borrow `opt` using `&opt` so that it is not consumed
    match &opt {
        Some(s) => println!("Some: {}", s), // Borrow the string inside `opt`
        None => println!("None!"),
    };

    println!("{:?}", opt); // `opt` can still be used here because it was not moved
}
// Context: In Rust, when pattern matching on an Option<T> or other types, ownership of the inner value may be moved depending on how you match it. If you need to reuse the original variable after the match block, you can borrow the value instead of taking ownership.
