// Solution: Use `_ => ()` to handle unmatched cases without performing any action
fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),    // Handle dice roll of 3
        7 => remove_fancy_hat(), // Handle dice roll of 7
        _ => (),                 // Catch-all pattern: do nothing for unmatched cases
    }
}

fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}

// Context: In Rust, match expressions must be exhaustive, meaning all possible cases must be handled. If you don't want to perform any action for unmatched cases, you can use _ => (), which evaluates to the unit value () and effectively does nothing.
// Unit Type (): The unit type represents an "empty" value or no meaningful action in Rust.
