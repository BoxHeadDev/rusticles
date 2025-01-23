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
fn pattern_matching_1() {
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

// Fix the `plus_one` match expression to handle all possible variants of `Option<i32>`
fn pattern_matching_2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1), // Handle the `Some` variant
                                // Add handling for the `None` variant
    }
}

// Add a catch-all pattern to handle all other dice rolls
fn pattern_matching_3() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(), // Specific case: dice roll is 3
        7 => remove_fancy_hat(), // Specific case: dice roll is 7
                               // Add a catch-all pattern here to call `move_player`
    }
}

// Add a catch-all pattern to handle all other dice rolls with `reroll``
fn pattern_matching_4() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(), // Specific case: dice roll is 3
        7 => remove_fancy_hat(), // Specific case: dice roll is 7
                               // Add a catch-all pattern here to call `reroll`
    }
}
fn reroll() {
    println!("Rerolling the dice!");
}

// Fix the error by ensuring all cases are handled
fn pattern_matching_5() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(), // Specific case: dice roll is 3
        7 => remove_fancy_hat(), // Specific case: dice roll is 7
                               // Add a catch-all pattern here to do nothing for unmatched cases
    }
}

fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}

fn move_player(num_spaces: u8) {
    println!("Moving player by {} spaces!", num_spaces);
}

// Fix the error by ensuring `opt` can still be used after the `match` block
fn pattern_matching_6_a() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match opt {
        Some(s) => println!("Some: {}", s), // This moves `s` out of `opt`
        None => println!("None!"),
    };

    println!("{:?}", opt); // This causes an error because `opt` was moved
}

// Fix the error so that the value inside `opt` is not consumed
fn pattern_matching_6_b() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match opt {
        Some(s) => println!("Some: {}", s), // This consumes `s`, making `opt` unusable
        None => println!("None!"),
    };

    println!("{:?}", opt); // This should still print the value inside `opt`
}

enum Location {
    Point(i32),
    Range(i32, i32),
}

// What will the output be?
fn pattern_matching_7() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::Point(_) => -1,
        Location::Range(_, n) => n,
        Location::Range(0, _) => 0,
        _ => -2,
    };
    println!("{n}");
}

// Fix the error by avoiding Moves in Pattern Matching
enum Either {
    Left(usize),
    Right(String),
}

fn pattern_matching_8() {
    let x = Either::Right(String::from("Hello world"));
    let value = match x {
        Either::Left(n) => n,
        Either::Right(s) => s.len(),
    };
    println!("{x:?} {value}");
}

// Refactor this function to use a match expression
fn decr_twice(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
    }
}

// Simplify the match expression using if let
fn pattern_matching_9() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // Do nothing for all other cases
    }
}

// Simplify the match expression using if let and else
fn pattern_matching_10(coin: Coin) {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1, // Increment count for all other coin types
    }

    println!("Count: {count}");
}

fn main() {
    // Execute the function to see if your changes worked!
    pattern_matching_1();
    pattern_matching_2();
    pattern_matching_3();
    pattern_matching_4();
    pattern_matching_5();
    pattern_matching_6_a();
    pattern_matching_6_b();
    pattern_matching_7();
    pattern_matching_8();
    pattern_matching_9();
    pattern_matching_10();

    assert_eq!(decr_twice(0), None);
    assert_eq!(decr_twice(1), None);
    assert_eq!(decr_twice(2), Some(0));
    assert_eq!(decr_twice(5), Some(3));
    println!("All tests passed!");
}
