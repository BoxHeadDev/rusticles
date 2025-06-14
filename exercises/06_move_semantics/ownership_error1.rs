// Fix this function to avoid returning a reference to a local variable!
// The goal is to return a valid `String` or a valid reference.
fn return_a_string() -> &String {
    let s = String::from("Hello world"); // `s` is created as a local variable.
    &s // ERROR: Returning a reference to `s`, which will be deallocated after the function ends.
}

fn main() {
    // Execute the function to see if your changes worked!
    let result = return_a_string();
    println!("{}", result);
}
