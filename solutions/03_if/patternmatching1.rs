#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// Solution: Modify the `Coin` enum to associate a `UsState` value with the `Quarter` variant
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // The `Quarter` variant now holds a `UsState`
}
// Context: In Rust, enums can hold data in their variants. This allows you to associate additional information with specific variants.

// Solution: Function to test the pattern matching
fn main() {
    value_in_cents(Coin::Penny); // Test with Coin::Penny
    value_in_cents(Coin::Quarter(UsState::Alabama)); // Test with a Quarter from Alabama
}

// Function to return the value of a coin in cents
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!"); // Print the special message for pennies
            1 // Return the value of a penny
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!"); // Print the state name for a quarter
            25 // Return the value of a quarter
        }
    }
}
// Context: In Rust, the match expression is a powerful tool for handling different enum variants. You can execute additional code for specific variants or bind values from enum variants to variables.
