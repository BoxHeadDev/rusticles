// Create enum for different kinds of IP addresses (V4, V6)
// IpAddrKind

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
    // let four = ??
    // let six = ??
}

// Implement instances of IpAddr using the IpAddrKind enum
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn enum_2() {
    // let home = ??

    // let loopback = ??
}

// Instead of using the enum with a struct
// put the data directly into each enum variant
enum IpAddr {
    V4,
    V6,
}

fn enum_3() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

// Store V4 addresses as four u8 values but still express V6 addresses as one String value
enum IpAddr {
    V4(String),
    V6(String),
}

fn enum_4() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

// Convert the following structs into a single enum Message
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Create call method for message to replace WriteMessage
fn enum_5() {
    let m = WriteMessage(String::from("hello"));
    m.call();
}

// Add types to the following variables
fn enum_6() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number = None;
}

// Fix the following error
fn enum_7() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
