fn main() {
    combine_strings();
    update_strings();
    combine_strings();
    index_strings();
    loop_strings();
}

fn create_strings() {
    // Create a new string
    let mut s = String::new();

    // Create a string from the string literal
    let data = "initial contents";
    let s = data.to_string();
    // let s = "initial contents".to_string();

    // Use the string function to create a string from the string literal
    let s = String::from("initial contents");
}

fn update_strings() {
    // Add "bar" to the string
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {s2}");

    // Add "l" to the following string
    let mut s = String::from("lo");
    s.push('l');
}

fn combine_strings() {
    // Add s2 to the end of s1
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // Combine all strings and assign to s
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; //
    let s = format!("{s1}-{s2}-{s3}"); // macro
}

fn index_strings() {
    // Will the following compile?
    // No, Strings are Vec<u8> so has same rules
    let s1 = String::from("hello");
    let h = s1[0];

    // What is the value of answer?
    let hello = String::from("Hola");
    let answer = &hello[0]; // 48  (UTF 8 encoding)

    // Assign the first 3 charaters of hello
    let firstThree = &hello[0..4];
}

fn loop_strings() {
    let hello = String::from("hello");

    // loop over hello and print each character
    for c in hello.chars() {
        println!("{c}");
    }

    // loop over hello and print each byte
    for b in hello.bytes() {
        println!("{b}");
    }
}
