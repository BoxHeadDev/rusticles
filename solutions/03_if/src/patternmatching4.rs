// Solution: Add a catch-all pattern to call `reroll`
fn main() {
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

fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}

// Context: In Rust, the _ pattern can be used in a match expression as a catch-all pattern when no further action is needed to capture the value. This is useful when you want to handle all other cases without requiring the value itself.
