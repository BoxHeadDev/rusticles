// Fix the code to ensure variables are properly declared and initialised before use.
// The goal is to correctly define `x` before passing it to the `read` function.
fn main() {
    read(x); // ERROR: `x` is used before being declared.
    let x = true; // `x` is declared here.
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}
