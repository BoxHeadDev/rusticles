fn main() {
    // Loop over v and add 50 to each element.
    let mut v = vec![100, 32, 57];

    for n_ref in &mut v {
        ??; // Hint: Use `*n_ref` to access and modify the value.
    }
}
