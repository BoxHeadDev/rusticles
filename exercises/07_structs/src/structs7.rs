#[derive(Debug)]
struct User {
    active: bool,       // Indicates if the user is active.
    username: String,   // The user's username.
    email: String,      // The user's email address.
    sign_in_count: u64, // The number of times the user has signed in.
}

// Implement a `build_user` function to create a User with default values for `active` and `sign_in_count`.


fn main() {
    // Use the builder function to create a `User` instance.
    // String::from("someone@example.com")
    // String::from("someusername123")
    let user = ??;

    println!("User: {:?}", user);
}

