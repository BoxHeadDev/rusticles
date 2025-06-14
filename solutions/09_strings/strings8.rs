fn main() {
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
