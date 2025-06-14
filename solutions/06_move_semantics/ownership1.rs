// Solution: The code is fixed by declaring `x` before it is used.
fn main() {
    let x = true; // Declare and initialise `x` before using it.
    read(x); // Pass `x` to the `read` function.
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}
// Context: The original code attempts to use the variable x before it is declared. Rust enforces strict rules ensuring variables are declared and initialised before use to prevent undefined behaviour.
