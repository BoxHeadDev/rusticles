fn main() {
    // Add s2 to the end of s1.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // `+` operator takes ownership of `s1` and appends `&s2`.

    // Combine all strings and assign to s.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Using the `+` operator.
    let s = s1 + "-" + &s2 + "-" + &s3;

    // Alternatively, using the `format!` macro (does not consume the strings).
    // let s = format!("{s1}-{s2}-{s3}");
}
// Context: In Rust, you can combine strings using the + operator or the format! macro.
// The + operator takes ownership of the first string and appends the second string (passed as a reference).
// The format! macro provides a convenient way to combine multiple strings without consuming any of them.
