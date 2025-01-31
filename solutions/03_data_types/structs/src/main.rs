#[derive(Debug)]
struct User {
    active: bool,       // Indicates if the user is active.
    username: String,   // The user's username.
    email: String,      // The user's email address.
    sign_in_count: u64, // The number of times the user has signed in.
}
// Context: Structs in Rust are used to group related data together. They provide a way to define custom data types with named fields.

fn struct_1() {
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

fn struct_2() {
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

fn struct_3() {
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

// Builder function to create a User with default values for `active` and `sign_in_count`.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,     // Default value.
        sign_in_count: 1, // Default value.
    }
}

fn struct_4() {
    // Use the builder function to create a `User` instance.
    let user = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    println!("User: {:?}", user);
}
// Context: A builder function is a convenient way to create instances of a struct. Instead of directly initialising the struct in multiple places, you can encapsulate the creation logic in a single function. This makes the code more maintainable and avoids repetitive struct initialisation code.

// Define a unit-like struct called `AlwaysEqual`.
struct AlwaysEqual;

fn struct_5() {
    // Create an instance of AlwaysEqual.
    let subject = AlwaysEqual;
}
// Context: Unit-like structs in Rust are structs without any fields. They are often used for specific purposes, such as marker types or when the struct itself conveys meaning without needing data.

struct User2 {
    active: bool,
    username: String, // Use `String` for ownership.
    email: String,    // Use `String` for ownership.
    sign_in_count: u64,
}

fn struct_6() {
    // Create a valid instance of User.
    let user6 = User2 {
        active: true,
        username: String::from("someusername123"), // Convert `&str` to `String`.
        email: String::from("someone@example.com"), // Convert `&str` to `String`.
        sign_in_count: 1,
    };
}
// Context: In Rust, you cannot directly use a reference (&str) in a struct unless the struct also contains a lifetime annotation. Instead, you should use an owned String type for fields that need to hold string data. This ensures the struct owns its data and avoids issues with dangling references.

// Define two tuple structs: `Color` and `Point`.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct_1() {
    // Create an instance of the `Color` tuple struct.
    let black = Color(0, 0, 0); // Represents black in RGB.

    // Create an instance of the `Point` tuple struct.
    let origin = Point(0, 0, 0); // Represents the origin in 3D space.
}
// Context: Tuple structs in Rust are a hybrid between tuples and traditional structs. They group related values together but use positional fields instead of named ones. Tuple structs are useful when you want a lightweight structure with less verbosity than named structs.

struct Color2(i32, i32, i32);

fn tuple_struct_2() {
    // Create an instance of the `Color` tuple struct.
    let black = Color2(0, 0, 0); // Represents black in RGB.

    // Destructure the `Color` instance into individual variables.
    let Color2(x, y, z) = black;

    println!("x: {x}, y: {y}, z: {z}");
}
// Context: Tuple structs in Rust can be destructured into individual variables for convenience. This is particularly useful when you need to access or manipulate the individual fields of a tuple struct.

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
