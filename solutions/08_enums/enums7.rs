// Solution:
enum IpAddr {
    V4(u8, u8, u8, u8), // The `V4` variant now holds four `u8` values for an IPv4 address.
    V6(String),         // The `V6` variant remains a `String` for an IPv6 address.
}

fn main() {
    // Create an instance of the `V4` variant with four `u8` values.
    let home = IpAddr::V4(127, 0, 0, 1);

    // Create an instance of the `V6` variant with an IPv6 address as a `String`.
    let loopback = IpAddr::V6(String::from("::1"));
}
// Context: Enums in Rust can hold different types of data in each variant, enabling you to model real-world data concisely. For example, an IPv4 address can be represented by four u8 values, while an IPv6 address might still be best represented as a single String. This allows you to differentiate and store the most appropriate data for each case.
