// Fix the code to correctly return a value from the loop
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            counter * 2; // This does not return a value. How can you fix it?
        }
    };

    println!("The result is {result}"); // Should print the result
}
