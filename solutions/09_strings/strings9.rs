fn main() {
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
