#[derive(Debug)]
struct User {
    active: bool,       // Indicates if the user is active.
    username: String,   // The user's username.
    email: String,      // The user's email address.
    sign_in_count: u64, // The number of times the user has signed in.
}

// Builder function to create a User with default values for `active` and `sign_in_count`.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,     // Default value.
        sign_in_count: 1, // Default value.
    }
}

fn main() {
    // Use the builder function to create a `User` instance.
    let user = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    println!("User: {:?}", user);
}
// Context: A builder function is a convenient way to create instances of a struct. Instead of directly initialising the struct in multiple places, you can encapsulate the creation logic in a single function. This makes the code more maintainable and avoids repetitive struct initialisation code.
