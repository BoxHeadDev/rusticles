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
