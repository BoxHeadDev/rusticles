// Solution:
enum IpAddrKind {
    V4, // Represents an IPv4 address
    V6, // Represents an IPv6 address
}
// Context: Enums in Rust allow you to define a type that can have a fixed set of variants. For example, an IP address can be either IPv4 or IPv6. By using an enum, you can model these different kinds of IP addresses in a structured way.

// Solution:
fn main() {
    let four = IpAddrKind::V4; // Instance of the `V4` variant.
    let six = IpAddrKind::V6; // Instance of the `V6` variant.
}
// Context: Once you've defined an enum, you can create instances of its variants using the :: syntax. This allows you to create values that represent the specific variants of the enum. For example, if you have an enum IpAddrKind with variants V4 and V6, you can create instances like IpAddrKind::V4.
