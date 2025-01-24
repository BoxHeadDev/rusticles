// Define an enum `IpAddrKind` to represent different kinds of IP addresses.
// Hint: The enum should have two variants: `V4` and `V6`.

// Create instances of each enum variant below.
fn enum_1() {
    // Hint: Use the `IpAddrKind` enum to create instances of `V4` and `V6`.
    // let four = ??;
    // let six = ??;
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Complete the implementation of `home` and `loopback` below by creating instances of the IpAddr struct by specifying both the kind (using the IpAddrKind enum) and the address.
fn enum_2() {
    // Hint: Use the `IpAddrKind` enum to specify the kind, and provide an IP address as a `String`.
    // let home = ??;

    // let loopback = ??;
}

// Modify the enum to store data directly within its variants.
enum IpAddr {
    V4, // Hint: Add a `String` to this variant.
    V6, // Hint: Add a `String` to this variant.
}

fn enum_3() {
    // Fix the code to create `IpAddr` instances with embedded data.
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

// Modify the enum to store an IPv4 address as four `u8` values and an IPv6 address as a `String`.
enum IpAddr {
    V4(String), // Hint: Change this to store four `u8` values.
    V6(String),
}

fn enum_4() {
    // Fix the code to use four `u8` values for the `V4` variant.
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

// Convert the following structs into variants of a single enum `Message`.
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Define the `Message` enum here, consolidating the above structs.

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// Refactor this code to use the `Message` enum instead of the standalone `WriteMessage` struct.
fn enum_5() {
    let m = WriteMessage(String::from("hello")); // Replace this with the `Message` enum.
    m.call(); // This should call the `call` method of the `Message` enum.
}

// Add type annotations to variables that use the Option enum to clarify their intended types.
fn enum_6() {
    let some_number = Some(5); // Hint: Specify the type of the number wrapped in `Some`.
    let some_char = Some('e'); // Hint: Specify the type of the character wrapped in `Some`.

    let absent_number = None; // Hint: Specify the type of the `Option` explicitly.
}

// Fix the following error by handling the `Option` type.
fn enum_7() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // `y` is an `Option<i8>`.

    let sum = x + y; // Hint: Extract the value from `y` before adding it to `x`.
}

fn main() {
    // Execute the function to see if your changes worked!
    enum_1();
    enum_2();
    enum_3();
    enum_4();
    enum_5();
    enum_6();
    enum_7();
}
