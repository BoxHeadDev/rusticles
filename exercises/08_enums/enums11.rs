// Fix the following error by handling the `Option` type.
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // `y` is an `Option<i8>`.

    let sum = x + y; // Hint: Extract the value from `y` before adding it to `x`.
}
