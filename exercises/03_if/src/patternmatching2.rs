// Fix the `plus_one` match expression to handle all possible variants of `Option<i32>`
fn main() {
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
