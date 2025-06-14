// Fix the code to ensure borrowing rules are respected.
// The goal is to safely borrow and mutate different parts of a tuple.
fn main() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = get_first(&name); // Immutable reference to `name.0`.
    name.1.push_str(", Esq."); // ERROR: Cannot mutate `name` while `first` exists.
    println!("{first} {}", name.1);
}

fn get_first(name: &(String, String)) -> &String {
    &name.0 // Borrow the first element of the tuple.
}
