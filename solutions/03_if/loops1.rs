// Solution: Use `break` to return a value from the loop
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Return `counter * 2` and exit the loop
        }
    };

    println!("The result is {result}"); // Prints "The result is 20"
}
// Context: In Rust, you can return a value from a loop by using the break statement with an associated value. This value is then assigned to a variable that is bound to the result of the loop.
