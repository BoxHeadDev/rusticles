// Solution: Add handling for the `None` variant
fn main() {
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
