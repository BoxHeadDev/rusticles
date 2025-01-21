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
    Quarter,
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
        // Print "Lucky penny!" as well
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Bind the quarter value to a variable
        Coin::Quarter => 25,
    }
}

fn pattern_matching_2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// Fix the following match expression
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
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
    }
}
fn move_player(num_spaces: u8) {}

// Add a catch all pattern using reroll
fn pattern_matching_4() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
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
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

// Fix the error
fn pattern_matching_6() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match opt {
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
    println!("{n}");
}

// Why does the following not compile?
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

// Update the following to use a match expression
fn decr_twice(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
    }
}

// Write more concise using IF expressions
fn pattern_matching_9() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}

// Write more concise using IF expressions
fn pattern_matching_10() {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
}
