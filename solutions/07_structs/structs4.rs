#[derive(Debug)]
struct User {
    active: bool,       // Indicates if the user is active.
    username: String,   // The user's username.
    email: String,      // The user's email address.
    sign_in_count: u64, // The number of times the user has signed in.
}
// Context: Structs in Rust are used to group related data together. They provide a way to define custom data types with named fields.

fn main() {
    // Create an instance of User.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Change the email to "anotheremail@example.com".
    user1.email = String::from("anotheremail@example.com");
}
// Context: In Rust, structs are used to group related data. You can create an instance of a struct by specifying values for each of its fields. Additionally, if a struct instance is mutable, you can update its fields after creation.
