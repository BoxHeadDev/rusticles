fn main() {
    // What do each of the following return below?
    let v = vec![1, 2, 3, 4, 5];

    // Accessing an out-of-bounds index using indexing causes a panic.
    // let does_not_exist = &v[100]; // Uncommenting this will panic at runtime.

    // Accessing an out-of-bounds index using `.get()` returns `None`.
    let does_not_exist = v.get(100); // Returns None.
    match does_not_exist {
        Some(value) => println!("Found value: {value}"),
        None => println!("Index out of bounds!"),
    }
}
// Context: Vectors in Rust provide two ways to access elements: indexing (v[100]) and the .get() method. Accessing an out-of-bounds index using indexing causes a runtime panic, while .get() safely returns None if the index does not exist. This ensures that you can handle errors gracefully when using .get().
