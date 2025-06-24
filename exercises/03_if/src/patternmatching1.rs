#[derive(Debug)] // so we can inspect the state in a minute
// Define an enum representing U.S. states
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// Modify the `Coin` enum to associate a `UsState` value with the `Quarter` variant
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Complete the `value_in_cents` function to handle the requirements
fn main() {
    value_in_cents(Coin::Penny);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // Print "Lucky penny!" when `Coin::Penny` is matched
        Coin::Penny => 1, // Add the necessary code here

        Coin::Nickel => 5,
        Coin::Dime => 10,

        // Print the state when `Coin::Quarter` is matched
        Coin::Quarter => 25, // Bind the state and complete the code here
    }
}
