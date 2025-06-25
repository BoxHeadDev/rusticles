// Mutating read only data
// name in helper function is an immutable reference
// The function is fixed by working with immutable references and cloning data when needed.

fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0]; // Immutable borrow of the first element.
    stringify_name_with_title(&name); // This function no longer mutates `name`.
    println!("{}", first); // Safe to use `first` because no mutation occurred.
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" "); // Use `join` to create a copy of the data in `name`.
    full.push_str(" Esq."); // Append " Esq." to the copied string.
    full // Return the new string.
}
// Context: The original code attempts to modify a vector (name) while an immutable reference (first) exists, which violates Rust’s borrowing rules. Your task is to fix the function so that it compiles and behaves correctly by avoiding simultaneous mutable and immutable borrowing.
