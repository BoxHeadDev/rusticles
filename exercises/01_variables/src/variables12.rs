// Fix the code to correctly demonstrate shadowing and scopes
fn main() {
    let x = 5;
    let x = x + 1; // Shadow the original `x`
    let x = x * 2; // Shadow `x` again in the inner scope

    println!("The value of x in the inner scope is: {x}"); // Should print 12
    println!("The value of x is: {x}"); // Should print 6
}
