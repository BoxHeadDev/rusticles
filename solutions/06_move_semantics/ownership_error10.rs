// Solution: The code is fixed by modifying a copy of the string instead of attempting to mutate the borrowed value.
fn main() {
    let name = String::from("Ferris");
    let name_ref = &name; // Immutable borrow of `name`.
    let phd_name = award_phd(name.clone()); // Clone `name` and pass the owned copy to be modified.
    println!("{}", name_ref); // Safe to use the original `name_ref` as `name` was not modified.
    println!("{}", phd_name); // Print the modified copy of the name with "Ph.D." appended.
}

fn award_phd(name: String) -> String {
    let mut name = name; // Take ownership of the cloned string.
    name.push_str(", Ph.D."); // Safely modify the owned string.
    name // Return the modified string.
}
// Context: The original code attempts to modify a borrowed string (name) by taking ownership of it inside the award_phd function. This violates Rust's borrowing rules and leads to potential memory safety issues, as the ownership of name is transferred while a reference (name_ref) to it still exists.
