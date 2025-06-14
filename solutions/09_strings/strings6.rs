fn main() {
    // Add "bar" to the string.
    let mut s = String::from("foo");
    let s2 = "bar";

    // Append a string slice (&str) `s2` to a String `s`
    s.push_str(s2);
    println!("s2 is {s2}"); // Outputs: s2 is bar

    // String allows modifications when declared as mutable
    let mut s = String::from("lo");
    // Append a single character "l" to a String `s`
    s.push('l');
}
// Context: Rust’s String type allows you to modify its contents using methods like .push_str() to append a string slice or .push() to append a single character. These methods let you efficiently build and update strings.
