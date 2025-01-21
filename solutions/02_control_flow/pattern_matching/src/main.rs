#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// Bind UsState values to Quarter
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    pattern_matching_1();
    pattern_matching_2();
    pattern_matching_3();
    pattern_matching_4();
    pattern_matching_5();
    pattern_matching_6();
    pattern_matching_7();
    pattern_matching_8();
    pattern_matching_9();
    pattern_matching_10();
}

fn pattern_matching_1() {
    value_in_cents(Coin::Penny);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn pattern_matching_2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// Fix the following match expression
// matches are exhaustive (all possibilities)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Add a catch all pattern using move_player
// move_player should take the value as an argument
fn pattern_matching_3() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}
fn move_player(num_spaces: u8) {}

// Add a catch all pattern using reroll
// reroll should take no arguments
fn pattern_matching_4() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}
fn reroll() {}

// Fix the error
// Don't run any code if nothing matches
fn pattern_matching_5() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // unit value (empty tuple)
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

// Fix the error
fn pattern_matching_6() {
    let opt: Option<String> = Some(String::from("Hello world"));

    // solution 1: opt became &opt
    match &opt {
        // Some(_) => println!("Some!"), // Solution 2: _ became s
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);
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
    println!("{n}"); // 5
}

// Why does the following not compile?
enum Either {
    Left(usize),
    Right(String),
}

// The match arm Either::Right(s) moves the field s, so x cannot be used in the println.
fn pattern_matching_8() {
    let x = Either::Right(String::from("Hello world"));
    let value = match x {
        Either::Left(n) => n,
        Either::Right(s) => s.len(),
    };
    println!("{x:?} {value}");
}

// Update the following to use a match expression
fn decr_twice(n: u32) -> Option<u32> {
    match n {
        0 => None,
        1 => None,
        n2 => Some(n2 - 2),
    }
}

// Write more concise using IF expressions
fn pattern_matching_9() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

// Write more concise using IF expressions
fn pattern_matching_10() {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
