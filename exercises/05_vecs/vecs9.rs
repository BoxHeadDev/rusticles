fn main() {
    // What is wrong with the following code?
    let v = vec![String::from("Hello ")];

    let mut s = v[0]; // Error: Cannot move out of a vector by indexing.
    s.push_str("world");
    println!("{s}");
}
