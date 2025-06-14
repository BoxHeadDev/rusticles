enum IpAddrKind {
    V4, // Represents an IPv4 address
    V6, // Represents an IPv6 address
}

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
