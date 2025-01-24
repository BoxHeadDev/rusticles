// Define the `User` struct with the specified fields.
// active: A boolean indicating if the user is active.
// username: A String representing the user’s username.
// email: A String representing the user’s email.
// sign_in_count: A u64 representing how many times the user has signed in.

fn struct_1() {
    // Create an instance of User.
    let mut user1 = ??; // Hint: Use the `User` struct and provide initial values for all fields.

    // Change the email to "anotheremail@example.com".
    ??; // Hint: Update the `email` field of `user1`.
}

fn struct_2() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Create an instance of User with the same values as user1,
    // but a different email address ("another@example.com").
    let user2 = ??; // Hint: Use the values from `user1` where possible.
}

fn struct_3() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Create an instance of User by spreading user1 values.
    let user3 = ??; // Hint: Use `..user1` to reuse fields from `user1`.
}


// Implement a `build_user` function.
fn struct_4() {
    // Use the builder function to create a `User` instance.
    // String::from("someone@example.com")
    // String::from("someusername123")
    let user = ??;

    println!("User: {:?}", user);
}


// Define a unit-like struct called `AlwaysEqual`.

fn struct_5() {
    // Create an instance of AlwaysEqual.
    let subject = ??; // Hint: Use the name of the unit-like struct to create an instance.
}

// Create a valid instance of User.
struct User {
    active: bool,
    username: &str, // Fix this field to use `String`.
    email: &str,    // Fix this field to use `String`.
    sign_in_count: u64,
}

fn struct_6() {
    let user6 = User {
        active: true,
        username: "someusername123", // Fix this to use `String`.
        email: "someone@example.com", // Fix this to use `String`.
        sign_in_count: 1,
    };
}


// Define two tuple structs: `Color` and `Point`.
// Each should have three `i32` fields.

fn tuple_struct_1() {
    // Create an instance of the `Color` tuple struct.
    let black = ??; // Hint: Use `Color` and provide three values.

    // Create an instance of the `Point` tuple struct.
    let origin = ??; // Hint: Use `Point` and provide three values.
}

struct Color(i32, i32, i32);

fn tuple_struct_2() {
    // Create an instance of the `Color` tuple struct.
    let black = ??; // Hint: Use `Color` and provide three values.

    // Destructure the `Color` instance into individual variables.
    let (x, y, z) = ??; // Hint: Use pattern matching to destructure.
}

fn main() {
    // Execute the function to see if your changes worked!
    struct_1();
    struct_2();
    struct_3();
    struct_4();
    struct_5();
    struct_6();

    tuple_struct_1();
    tuple_struct_2();
}
