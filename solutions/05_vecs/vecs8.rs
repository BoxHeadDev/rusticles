fn main() {
    // Loop over v and add 50 to each element.
    let mut v = vec![100, 32, 57];

    for n_ref in &mut v {
        *n_ref += 50; // Dereference `n_ref` to modify the value.
    }

    println!("{:?}", v); // Outputs: [150, 82, 107]
}
// Context: In Rust, you can iterate over a vector and mutate its elements by borrowing them mutably. This allows you to modify the values in-place without creating a new vector. To change the value of a mutable reference, you must use the * operator to dereference it.
