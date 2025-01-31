fn create_strings() {
    // Creating an empty string.
    let mut s = String::new();

    // Converting String Slices.
    let data = "initial contents"; // &str (reference, static, immutable)
    let y = data.to_string(); // String (owned, dynamic, mutable)

    // Use the string function to create a string from the string literal.
    let z = String::from("initial contents");
}
// Context: In Rust, strings come in two primary types: &str (string slices) and String (owned, growable strings). While string slices are static and cannot be modified, the String type provides the flexibility to create and manipulate strings dynamically.

fn update_strings() {
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

fn combine_strings() {
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

fn index_strings() {
    // No, Strings are UTF-8 encoded and do not support direct indexing like `Vec<u8>`.
    let s1 = String::from("hello");
    let h = &s1[0..1]; // string slicing

    // What is the value of answer? Why?
    let hello = String::from("Hola");
    // UTF-8 strings cannot be directly indexed, but we can use slicing.
    let answer = &hello[0..1]; // This returns "H", the first character.

    // Assign the first 3 characters of hello.
    let firstThree = &hello[0..3]; // "Hol", using slicing.
}
// Context: In Rust, String is a collection of UTF-8 encoded bytes, which means indexing directly into a String is not allowed. Instead, you must use slicing with ranges to access specific parts of a String. This avoids runtime errors caused by invalid UTF-8 character boundaries.

fn loop_strings() {
    let hello = String::from("hello");

    // Loop over hello and print each character.
    for c in hello.chars() {
        println!("{c}"); // Prints each character: h, e, l, l, o
    }

    // Loop over hello and print each byte.
    for b in hello.bytes() {
        println!("{b}"); // Prints each byte: 104, 101, 108, 108, 111
    }
}
// Context: Strings in Rust are UTF-8 encoded, meaning each character (char) may span multiple bytes. Rust provides two methods, .chars() and .bytes(), to iterate over the contents of a String. The .chars() method iterates over Unicode scalar values (individual characters), while .bytes() iterates over the raw bytes of the string.

fn main() {
    // Execute the function to see if your changes worked!
    combine_strings();
    update_strings();
    combine_strings();
    index_strings();
    loop_strings();
}
