// Add a catch-all pattern to handle all other dice rolls
fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(), // Specific case: dice roll is 3
        7 => remove_fancy_hat(), // Specific case: dice roll is 7
                               // Add a catch-all pattern here to call `move_player`
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
