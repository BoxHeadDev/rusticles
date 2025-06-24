// Fix the code to respect Rust's borrowing rules and avoid ownership issues.
fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0]; // Immutable borrow of the first element.
    stringify_name_with_title(&name); // Attempt to modify `name` here.
    println!("{}", first); // ERROR: Immutable reference is still in use here!
}

// Ideally, the function should create a full name like:
// ["Ferris", "Jr."] => "Ferris Jr. Esq."
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq.")); // ERROR: Cannot mutate `name` while borrowed immutably.
    let full = name.join(" "); // Join the vector elements into a string.
    full
}
