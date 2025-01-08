// Create a user struct
// Fields: active, username, email, sign_in_count
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

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
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Change the email to a different string
    user1.email = String::from("anotheremail@example.com");
}

fn struct_2() {
    // Create an instance of User with the same values as user1
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

fn struct_3() {
    // Create an instance of User by spreading user1 values
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn struct_4() {
    // Create a builder function for User
    build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    )
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// unit-like struct
struct AlwaysEqual;

fn struct_5() {
    let subject = AlwaysEqual;
}

// Fix the following
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn struct_6() {
    let user6 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct_1() {
    // Create instances of Color and Point struct tuples
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn tuple_struct_2() {
    // Add each of the Color values to their own variable
    let black = Color(0, 0, 0);
    let (x, y, z) = black;
}
