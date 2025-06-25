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
    let user3 = User {
        email: String::from("another@example.com"), // New email.
        ..user1                                     // Reuse remaining fields from user1.
    };
}
// Context: Rust provides a concise way to create a new struct instance by reusing fields from an existing instance. This is called the struct update syntax (..). It allows you to specify only the fields you want to modify while copying the remaining fields from another instance.
