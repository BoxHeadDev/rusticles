// Solution: Add a catch-all pattern to call `move_player`
fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),        // Handle dice roll of 3
        7 => remove_fancy_hat(),     // Handle dice roll of 7
        other => move_player(other), // Catch-all pattern: handle all other dice rolls
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

// Context: In Rust, a match expression must account for all possible cases. When you don't explicitly handle all values, you can use a catch-all pattern (e.g., a variable like other or _) to ensure exhaustiveness.
