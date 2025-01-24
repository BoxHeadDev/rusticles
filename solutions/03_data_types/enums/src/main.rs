// Solution:
enum IpAddrKind {
    V4, // Represents an IPv4 address
    V6, // Represents an IPv6 address
}
// Context: Enums in Rust allow you to define a type that can have a fixed set of variants. For example, an IP address can be either IPv4 or IPv6. By using an enum, you can model these different kinds of IP addresses in a structured way.

// Solution:
fn enum_1() {
    let four = IpAddrKind::V4; // Instance of the `V4` variant.
    let six = IpAddrKind::V6; // Instance of the `V6` variant.
}
// Context: Once you've defined an enum, you can create instances of its variants using the :: syntax. This allows you to create values that represent the specific variants of the enum. For example, if you have an enum IpAddrKind with variants V4 and V6, you can create instances like IpAddrKind::V4.

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Solution:
fn enum_2() {
    // Create an instance of the `IpAddr` struct for an IPv4 address.
    let home = IpAddr {
        kind: IpAddrKind::V4,               // Specify the kind as IPv4.
        address: String::from("127.0.0.1"), // Assign the IPv4 address.
    };

    // Create an instance of the `IpAddr` struct for an IPv6 address.
    let loopback = IpAddr {
        kind: IpAddrKind::V6,         // Specify the kind as IPv6.
        address: String::from("::1"), // Assign the IPv6 address.
    };
}
// Context: Enums in Rust can be used in combination with structs to create more expressive and structured data types. For example, you can represent an IP address as a struct that contains both the kind of IP (V4 or V6) and the actual address as a String.

// Solution:
enum IpAddr {
    V4(String), // The `V4` variant holds an IPv4 address as a `String`.
    V6(String), // The `V6` variant holds an IPv6 address as a `String`.
}

fn enum_3() {
    // Create an instance of the `V4` variant with an IPv4 address.
    let home = IpAddr::V4(String::from("127.0.0.1"));

    // Create an instance of the `V6` variant with an IPv6 address.
    let loopback = IpAddr::V6(String::from("::1"));
}
// Context: Enums in Rust can store data directly within their variants, making them more expressive and eliminating the need for additional structs in some cases. For example, an IP address can be represented as an enum where each variant (V4 or V6) directly holds the address as a String.

// Solution:
enum IpAddr {
    V4(u8, u8, u8, u8), // The `V4` variant now holds four `u8` values for an IPv4 address.
    V6(String),         // The `V6` variant remains a `String` for an IPv6 address.
}

fn enum_4() {
    // Create an instance of the `V4` variant with four `u8` values.
    let home = IpAddr::V4(127, 0, 0, 1);

    // Create an instance of the `V6` variant with an IPv6 address as a `String`.
    let loopback = IpAddr::V6(String::from("::1"));
}
// Context: Enums in Rust can hold different types of data in each variant, enabling you to model real-world data concisely. For example, an IPv4 address can be represented by four u8 values, while an IPv6 address might still be best represented as a single String. This allows you to differentiate and store the most appropriate data for each case.

// Solution:
enum Message {
    Quit,                       // Unit variant
    Move { x: i32, y: i32 },    // Struct-like variant
    Write(String),              // Tuple-like variant
    ChangeColor(i32, i32, i32), // Tuple-like variant
}

impl Message {
    // Implement a method for the `Message` enum.
    fn call(&self) {
        // Match on the enum to handle the `Write` variant.
        match self {
            Message::Write(text) => {
                println!("Message: {}", text); // Print the message stored in the `Write` variant.
            }
        }
    }
}
// Context: In Rust, enums can store multiple data types and structures within their variants, making them a versatile way to represent different kinds of messages or states. Instead of having multiple structs, you can consolidate them into a single enum with various variants.

// Solution:
fn enum_5() {
    // Create an instance of the `Message` enum with the `Write` variant.
    let m = Message::Write(String::from("hello"));

    // Call the `call` method on the enum instance.
    m.call(); // Output: "Message: hello"
}
// Context: Enums in Rust can have methods, just like structs. These methods can operate on specific variants of the enum.

// Solution: Adding type annotations to clarify the type of value wrapped in `Some` and `None`.
fn enum_6() {
    let some_number: Option<i8> = Some(5); // `Option<i8>` wraps an 8-bit integer.
    let some_char: Option<char> = Some('e'); // `Option<char>` wraps a character.

    let absent_number: Option<i32> = None; // Explicitly define the type of the `None` variant as `Option<i32>`.
}
// Context: In Rust, the Option enum is used to represent a value that can either exist (Some) or be absent (None). However, when using Option, Rust requires you to specify the type of the value that might be present. This helps the compiler ensure type safety.

// Solution: extract the value from the `Option<i8>` type.
fn enum_7() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // `y` is an `Option<i8>`.

    // Use `.unwrap()` to extract the value from the `Option` for this example.
    let y_value = y.unwrap(); // This will panic if `y` is `None`.

    // Bonus: Provide a default value if `y` is `None`.
    // let y_value = y.unwrap_or(0);

    let sum = x + y_value; // Now you can safely add the two `i8` values.
    println!("The sum is: {}", sum); // Output: The sum is: 10
}
// Context: The Option enum is used to represent a value that may or may not exist, but it cannot be directly used in arithmetic operations without first handling the Option type. In Rust, you need to extract the value inside the Option using pattern matching or helper methods like .unwrap() (for learning purposes) to convert the Option into its inner type.

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
