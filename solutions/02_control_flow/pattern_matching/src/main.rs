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
fn pattern_matching_1() {
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

// Solution: Add handling for the `None` variant
fn pattern_matching_2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // Handle the `None` variant by returning `None`
        Some(i) => Some(i + 1), // Handle the `Some` variant by adding 1 to the value
    }
}
// Context: In Rust, match expressions must be exhaustive, meaning they need to account for all possible variants of the value being matched. The Option<T> enum has two variants: Some(T) and None.

// Solution: Add a catch-all pattern to call `move_player`
fn pattern_matching_3() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),        // Handle dice roll of 3
        7 => remove_fancy_hat(),     // Handle dice roll of 7
        other => move_player(other), // Catch-all pattern: handle all other dice rolls
    }
}
// Context: In Rust, a match expression must account for all possible cases. When you don't explicitly handle all values, you can use a catch-all pattern (e.g., a variable like other or _) to ensure exhaustiveness.

// Solution: Add a catch-all pattern to call `reroll`
fn pattern_matching_4() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),    // Handle dice roll of 3
        7 => remove_fancy_hat(), // Handle dice roll of 7
        _ => reroll(),           // Catch-all pattern: handle all other dice rolls
    }
}
fn reroll() {
    println!("Rerolling the dice!");
}
// Context: In Rust, the _ pattern can be used in a match expression as a catch-all pattern when no further action is needed to capture the value. This is useful when you want to handle all other cases without requiring the value itself.

// Solution: Use `_ => ()` to handle unmatched cases without performing any action
fn pattern_matching_5() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),    // Handle dice roll of 3
        7 => remove_fancy_hat(), // Handle dice roll of 7
        _ => (),                 // Catch-all pattern: do nothing for unmatched cases
    }
}
// Context: In Rust, match expressions must be exhaustive, meaning all possible cases must be handled. If you don't want to perform any action for unmatched cases, you can use _ => (), which evaluates to the unit value () and effectively does nothing.
// Unit Type (): The unit type represents an "empty" value or no meaningful action in Rust.

fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}

fn move_player(num_spaces: u8) {
    println!("Moving player by {} spaces!", num_spaces);
}

// Solution: Borrow `opt` in the `match` statement to avoid moving ownership
fn pattern_matching_6_a() {
    let opt: Option<String> = Some(String::from("Hello world"));

    // Borrow `opt` using `&opt` so that it is not consumed
    match &opt {
        Some(s) => println!("Some: {}", s), // Borrow the string inside `opt`
        None => println!("None!"),
    };

    println!("{:?}", opt); // `opt` can still be used here because it was not moved
}
// Context: In Rust, when pattern matching on an Option<T> or other types, ownership of the inner value may be moved depending on how you match it. If you need to reuse the original variable after the match block, you can borrow the value instead of taking ownership.

// Solution: Use `_` to ignore the value or `ref` to borrow it
fn pattern_matching_6() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        // Match `opt` by reference to avoid consuming it
        Some(_) => println!("Some!"), // Ignore the value with `_`
        None => println!("None!"),
    };

    println!("{:?}", opt); // `opt` is still usable since it wasn't consumed
}
// Context: When matching on an Option<String>, the Some(s) pattern moves the value out of the Option, consuming it. If you want to keep the value in the Option while still matching it, you can use _ to ignore the value or match it by reference (Some(ref s)).

// Solution: Corrected match with reordered patterns
enum Location {
    Point(i32),
    Range(i32, i32),
}

fn pattern_matching_7() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::Point(_) => -1,   // Match `Point`
        Location::Range(0, _) => 0, // Match `Range(0, _)` before generic `Range(_, n)`
        Location::Range(_, n) => n, // Match generic `Range(_, n)`
        _ => -2,                    // Default case (not needed here)
    };
    println!("{n}"); // Output will be `0` with the corrected code
                     // In the original code, the output is 5 due to pattern ordering
}
// Context: The match expression evaluates patterns in the order they are written. The third pattern (Location::Range(0, _) => 0) is unreachable. The second pattern already matches all Location::Range variants, so it is impossible for the third pattern to ever be executed. Rust's compiler should provide a warning about unreachable code here.

// Solution: use borrowing to avoid moving the value of x
#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String),
}

fn pattern_matching_8() {
    let x = Either::Right(String::from("Hello world"));

    // Borrow x to avoid moving its content
    let value = match &x {
        Either::Left(n) => *n,       // Use dereference to get the value for Left
        Either::Right(s) => s.len(), // Borrow the String to calculate its length
    };
    println!("{x:?} {value}");
}
// Context: The code does not compile because the match expression moves the value of the String stored in the Either::Right variant. Once the value is moved, the x variable is no longer valid for use, causing the compilation error.

// Solution: Refactored function using a match expression
fn decr_twice(n: u32) -> Option<u32> {
    match n {
        0 => None,          // If n is 0, return None
        1 => None,          // If n is 1, return None
        n2 => Some(n2 - 2), // For any other value, return n - 2 wrapped in Some
    }
}
// Context: This approach is more idiomatic in Rust because match clearly shows the branching logic based on the value of n.

// Solution: Refactored function using if let
fn pattern_matching_9() {
    let config_max = Some(3u8);

    // Use if let to handle the Some case concisely
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
// Context: if let is a concise and idiomatic way to handle cases where you only care about matching one specific pattern and want to ignore the rest.

// Refactored function using if let and else
fn pattern_matching_10(coin: Coin) {
    let mut count = 0;

    // Use if let to handle the Quarter case and else for the fallback
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1; // Increment count for all other coin types
    }

    println!("Count: {count}");
}
// Context: if let is a concise and idiomatic way to handle cases where you only care about matching one specific pattern and want to ignore the rest.

fn main() {
    // Execute the function to see if your changes worked!
    pattern_matching_1();
    pattern_matching_2();
    pattern_matching_3();
    pattern_matching_4();
    pattern_matching_5();
    pattern_matching_6();
    pattern_matching_7();
    pattern_matching_8();
    pattern_matching_9();

    // pattern_matching_10(Coin::Dime);
    pattern_matching_10(Coin::Quarter(UsState::Alaska));

    assert_eq!(decr_twice(0), None);
    assert_eq!(decr_twice(1), None);
    assert_eq!(decr_twice(2), Some(0));
    assert_eq!(decr_twice(5), Some(3));
    println!("All tests passed!");
}
