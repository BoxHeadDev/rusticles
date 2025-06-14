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

// Refactored function using if let and else
fn pattern_matching(coin: Coin) {
    let mut count = 0;

    // Use if let to handle the Quarter case and else for the fallback
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1; // Increment count for all other coin types
    }

    println!("Count: {count}");
}

fn main() {
    // pattern_matching(Coin::Dime);
    pattern_matching(Coin::Quarter(UsState::Alaska));
}

// Context: if let is a concise and idiomatic way to handle cases where you only care about matching one specific pattern and want to ignore the rest.
