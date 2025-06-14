// Solution: The code is modified to safely demonstrate ownership transfer.
fn main() {
    let a = Box::new([0; 1_000_000]); // A heap-allocated array.
    println!("a is created, and it owns the array.");

    let b = a; // Ownership of the array is moved from `a` to `b`.
    println!("Ownership of the array is moved to b.");

    // `a` can no longer be used here; uncommenting the next line will cause a compile error:
    // println!("{:?}", a);

    // `b` now owns the array and can be used:
    println!("b now owns the array, and its length is {}", b.len());
}
// Context: The original code demonstrates that when a (a Box containing a large array) is assigned to b, ownership of the heap-allocated memory is moved to b. After this, a is no longer valid, and attempting to use a will result in a compilation error.
