fn main() {
    // Loop over v and print each element after adding 1.
    let v = vec![100, 32, 57];

    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1; // Dereference `n_ref` to access its value.
        println!("{n_plus_one}");
    }
}
// Context: In Rust, when iterating over a vector, you can borrow its elements using a reference to avoid consuming the vector. This allows the vector to remain available for further use after the iteration. To access the actual value of a borrowed element, you need to dereference it using the * operator.
