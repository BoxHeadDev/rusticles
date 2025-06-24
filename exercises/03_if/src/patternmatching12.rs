#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Simplify the match expression using if let and else
fn pattern_matching(coin: Coin) {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1, // Increment count for all other coin types
    }

    println!("Count: {count}");
}

fn main() {
    // pattern_matching(Coin::Dime);
    pattern_matching(Coin::Quarter(UsState::Alaska));
}
