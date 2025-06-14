// Fix the error so that the value inside `opt` is not consumed
fn main() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match opt {
        Some(s) => println!("Some: {}", s), // This consumes `s`, making `opt` unusable
        None => println!("None!"),
    };

    println!("{:?}", opt); // This should still print the value inside `opt`
}
