// Modify the enum to store an IPv4 address as four `u8` values and an IPv6 address as a `String`.
enum IpAddr {
    V4(String), // Hint: Change this to store four `u8` values.
    V6(String),
}

fn main() {
    // Fix the code to use four `u8` values for the `V4` variant.
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
