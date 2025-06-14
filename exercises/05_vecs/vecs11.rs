fn main() {
    // Does the following compile?
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        v.push(*i); // Mutating the vector while iterating over it.
    }
    println!("{} {} {}", v[3], v[4], v[5]);
}
