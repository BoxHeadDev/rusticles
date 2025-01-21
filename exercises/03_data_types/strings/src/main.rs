fn main() {
    combine_strings();
    update_strings();
    combine_strings();
    index_strings();
    loop_strings();
}

fn create_strings() {
    // Create a new string


    // Create a string from the string literal
    let data = "initial contents";
    let s = ??;

    // Use the string function to create a string from the string literal
    let s = "initial contents";
}

fn update_strings() {
    // Add "bar" to the string
    let mut s = String::from("foo");
    let s2 = "bar";
    println!("s2 is {s2}");

    // Add "l" to the following string
    let mut s = String::from("lo");
}

fn combine_strings() {
    // Add s2 to the end of s1
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = ??;

    // Combine all strings and assign to s
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = ??;
}

fn index_strings() {
    // Will the following compile?
    let s1 = String::from("hello");
    let h = s1[0];

    // What is the value of answer?
    let hello = String::from("Hola");
    let answer = &hello[0];

    // Assign the first 3 charaters of hello
    let firstThree = ??;
}

fn loop_strings() {
    let hello = String::from("hello");

    // loop over hello and print each character
    // println!("{c}");

    // loop over hello and print each byte
    // println!("{b}");
}
