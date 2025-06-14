// Fix or explain the behavior of the code
fn main() {
    let mut x: u32 = 1; // Outer variable `x` is mutable
    {
        let mut x = x; // Shadow the outer `x` with a new mutable variable
        x += 2; // Modify the inner `x`
    }
    println!("{x}"); // What value will this print? Fix or explain!
}
