// Solution: Use `_` to ignore the value or `ref` to borrow it
fn main() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        // Match `opt` by reference to avoid consuming it
        Some(_) => println!("Some!"), // Ignore the value with `_`
        None => println!("None!"),
    };

    println!("{:?}", opt); // `opt` is still usable since it wasn't consumed
}
// Context: When matching on an Option<String>, the Some(s) pattern moves the value out of the Option, consuming it. If you want to keep the value in the Option while still matching it, you can use _ to ignore the value or match it by reference (Some(ref s)).
