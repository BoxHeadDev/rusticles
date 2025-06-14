// Solution:
enum IpAddr2 {
    V4(String), // The `V4` variant holds an IPv4 address as a `String`.
    V6(String), // The `V6` variant holds an IPv6 address as a `String`.
}

fn enum_3() {
    // Create an instance of the `V4` variant with an IPv4 address.
    let home = IpAddr2::V4(String::from("127.0.0.1"));

    // Create an instance of the `V6` variant with an IPv6 address.
    let loopback = IpAddr2::V6(String::from("::1"));
}
// Context: Enums in Rust can store data directly within their variants, making them more expressive and eliminating the need for additional structs in some cases. For example, an IP address can be represented as an enum where each variant (V4 or V6) directly holds the address as a String.
