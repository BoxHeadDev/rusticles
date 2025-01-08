// Create a user struct
// Fields: active, username, email, sign_in_count

fn main() {
    struct_1();
    struct_2();
    struct_3();
    struct_4();
    struct_5();
    struct_6();

    tuple_struct_1();
    tuple_struct_2();
}

fn struct_1() {
    // Create an instance of User
    // let mut user1 = ??

    // Change the email to a different string
    // anotheremail@example.com
}

fn struct_2() {
    // Create an instance of User with the same values as user1
    // let user2 = ??
}

fn struct_3() {
    // Create an instance of User by spreading user1 values
    // let user3 = ??
}

fn struct_4() {
    // Create a builder function for User
    build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    )
}

fn build_user(email: String, username: String) -> User {}

// Create a unit-like struct AlwaysEqual

fn struct_5() {
    let subject = AlwaysEqual;
}

// Fix the following
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn struct_6() {
    let user6 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}

// Create a struct tuple called Color and Point (i32)

fn tuple_struct_1() {
    // Create instances of Color and Point struct tuples
    // let black = ??
    // let origin = ??
}

fn tuple_struct_2() {
    // Create instance of Color using struct tuple
    // let black = ??

    // Add each of the Color values to their own variable
}
