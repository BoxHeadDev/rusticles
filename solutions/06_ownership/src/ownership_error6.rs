// Solution: The code is fixed by ensuring no immutable reference exists while mutating the tuple.
fn main() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = name.0.clone(); // Clone the first element to create an owned value.
    name.1.push_str(", Esq."); // Now safe to mutate `name.1` because `first` is independent.
    println!("{first} {}", name.1);
}

fn get_first(name: &(String, String)) -> &String {
    &name.0 // This function remains unchanged as it borrows immutably.
}
// Context: The original code creates an immutable reference (first) to one part of a tuple (name.0) while trying to mutate another part of the same tuple (name.1). This violates Rust's borrowing rules because you cannot have an active immutable reference while also mutating the same data structure.
