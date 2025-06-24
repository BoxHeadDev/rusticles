// Fix the error by ensuring all cases are handled
fn main() {
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
