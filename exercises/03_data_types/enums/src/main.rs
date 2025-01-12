// Create enum for different kinds of IP addresses (V4, V6)
// IpAddrKind
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    enum_1();
    enum_2();
    enum_3();
    enum_4();
    enum_5();
    enum_6();
    enum_7();
}

// Create instances of each enum variant
fn enum_1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// Implement instances of IpAddr usingthe IpAddrKind enum
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn enum_2() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// Instead of using the enum with a struct
// put the data directly into each enum variant
enum IpAddr {
    V4(String),
    V6(String),
}

fn enum_3() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

// Store V4 addresses as four u8 values but still express V6 addresses as one String value
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum_4() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

// Convert the following structs into a single enum Message
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// Create call method for message to replace WriteMessage
fn enum_5() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

// Add types to the following variables
fn enum_6() {
    let some_number: Option<i8> = Some(5);
    let some_char: Option<i8> = Some('e');

    let absent_number: Option<i32> = None;
}

// Rust cannot add two different types i8 and Option<i8>
fn enum_7() {
    let x: i8 = 5;
    let y: i8 = Some(5); // Remove Option type

    let sum = x + y;
}
