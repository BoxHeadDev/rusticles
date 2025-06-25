// Fix the code to respect borrowing rules and ensure safe usage of the string.
// The goal is to properly handle the `name` string without transferring ownership.
fn main() {
    let name = String::from("Ferris");
    let name_ref = &name; // Immutable borrow of `name`.
    award_phd(&name); // Attempt to modify `name` by taking ownership.
    println!("{}", name_ref); // ERROR: `name_ref` might be invalidated.
}

fn award_phd(name: &String) {
    let mut name = *name; // ERROR: Takes ownership of `name`, causing conflicts.
    name.push_str(", Ph.D."); // Attempt to modify `name`.
}
