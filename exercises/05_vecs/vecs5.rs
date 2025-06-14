fn main() {
    // What do each of the following return below?
    let v = vec![1, 2, 3, 4, 5];

    // Access an out-of-bounds index using indexing.
    let does_not_exist = &v[100]; // What happens here?

    // Access an out-of-bounds index using `.get()`.
    let does_not_exist = v.get(100); // What does this return?
}
