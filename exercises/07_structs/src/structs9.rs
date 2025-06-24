// Create a valid instance of User.
struct User {
    active: bool,
    username: &str, // Fix this field to use `String`.
    email: &str,    // Fix this field to use `String`.
    sign_in_count: u64,
}

fn main() {
    let user6 = User {
        active: true,
        username: "someusername123",  // Fix this to use `String`.
        email: "someone@example.com", // Fix this to use `String`.
        sign_in_count: 1,
    };
}
