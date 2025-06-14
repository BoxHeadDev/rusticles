#[derive(Debug)]
struct User {
    active: bool,       // Indicates if the user is active.
    username: String,   // The user's username.
    email: String,      // The user's email address.
    sign_in_count: u64, // The number of times the user has signed in.
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Create an instance of User by spreading user1 values.
    let user3 = ??; // Hint: Use `..user1` to reuse fields from `user1`.
}
