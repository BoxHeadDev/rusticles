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

    // Create an instance of User with the same values as user1, except for the email.
    let user2 = User {
        active: user1.active,
        username: user1.username.clone(), // Clone the username to reuse it.
        email: String::from("another@example.com"), // Use a new email.
        sign_in_count: user1.sign_in_count,
    };
}
// Context: In Rust, you can create a new instance of a struct by reusing fields from an existing instance. This technique reduces repetition and improves code readability. However, when reusing fields that involve ownership (e.g., String), you must ensure that ownership rules are followed.
