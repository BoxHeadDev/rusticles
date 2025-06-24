// Fix the error by ensuring `opt` can still be used after the `match` block
fn main() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match opt {
        Some(s) => println!("Some: {}", s), // This moves `s` out of `opt`
        None => println!("None!"),
    };

    println!("{:?}", opt); // This causes an error because `opt` was moved
}
