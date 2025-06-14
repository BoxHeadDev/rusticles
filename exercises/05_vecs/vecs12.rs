fn main() {
    // What is the output?
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        v2.push(i); // Hint: Add a mutable reference to the vector.
    }

    *v2[0] = 5; // Hint: Modify the first element through the reference.
    let a = *v2[0];
    let b = v[0];

    println!("{a} {b}"); // What values will be printed?
}
