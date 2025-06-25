struct User2 {
    active: bool,
    username: String, // Use `String` for ownership.
    email: String,    // Use `String` for ownership.
    sign_in_count: u64,
}

fn main() {
    // Create a valid instance of User.
    let user6 = User2 {
        active: true,
        username: String::from("someusername123"), // Convert `&str` to `String`.
        email: String::from("someone@example.com"), // Convert `&str` to `String`.
        sign_in_count: 1,
    };
}
// Context: In Rust, you cannot directly use a reference (&str) in a struct unless the struct also contains a lifetime annotation. Instead, you should use an owned String type for fields that need to hold string data. This ensures the struct owns its data and avoids issues with dangling references.
