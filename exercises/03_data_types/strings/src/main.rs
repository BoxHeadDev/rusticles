fn create_strings() {
    // Create an empty string.
    let s = ??; // Hint: Use `String::new()`.

    // Create a string from the string literal.
    let data = "initial contents";
    let y = ??; // Hint: Use `.to_string()` on `data`.

    // Use the string function to create a string from the string literal.
    let z = ??; // Hint: Use `String::from`.
}

fn update_strings() {
    // Add "bar" to the string.
    let mut s = String::from("foo");
    let s2 = "bar";
    // Hint: Use a method to append `s2` to `s`.
    println!("s2 is {s2}");

    // Add "l" to the following string.
    let mut s = String::from("lo");
    // Hint: Use a method to append a single character to `s`.
}

fn combine_strings() {
    // Add s2 to the end of s1.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = ??; // Hint: Use the `+` operator.

    // Combine all strings and assign to s.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = ??; // Hint: Use the `+` operator or the `format!` macro.
}

fn index_strings() {
    // Will the following compile? Why or why not?
    let s1 = String::from("hello");
    let h = s1[0]; // Fix this to work correctly.

    // What is the value of answer? Why?
    let hello = String::from("Hola");
    let answer = &hello[0]; // Fix this to return the correct first character.

    // Assign the first 3 characters of hello.
    let firstThree = ??; // Hint: Use slicing with UTF-8 boundaries.
}

fn loop_strings() {
    let hello = String::from("hello");

    // Loop over hello and print each character.
    // Hint: Use `.chars()`.

    // Loop over hello and print each byte.
    // Hint: Use `.bytes()`.
}

fn main() {
    // Execute the function to see if your changes worked!
    combine_strings();
    update_strings();
    combine_strings();
    index_strings();
    loop_strings();
}
