fn main() {
    // Loop over v and print each element after adding 1.
    let v = vec![100, 32, 57];

    for n_ref in &v {
        let n_plus_one: i32 = ??; // Hint: Dereference `n_ref` to access its value.
        println!("{n_plus_one}");
    }
}

