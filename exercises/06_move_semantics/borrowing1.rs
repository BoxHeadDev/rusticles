// Fix this code to make it compile!
// The goal is to use borrowing to avoid ownership errors.
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2); // ERROR: Ownership issues here!
    let s = format!("{} {}", m1, m2); // ERROR: m1 and m2 are no longer accessible!
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}
