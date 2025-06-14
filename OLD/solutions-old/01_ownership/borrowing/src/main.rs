// Solution: This version fixes the ownership issues by using references.
fn borrowing_1() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // Borrow `m1` and `m2` instead of transferring ownership.
    let s = format!("{} {}", m1, m2); // `m1` and `m2` are still accessible here!
}

fn greet(g1: &String, g2: &String) {
    // Accept references instead of taking ownership of `g1` and `g2`.
    println!("{} {}!", g1, g2);
}
// Context: Borrowing allows you to pass references to functions without transferring ownership, enabling you to reuse variables after the function call.

// Solution: This version fixes all issues by correctly using dereferencing and borrowing.
fn borrowing_2() {
    let mut x: Box<i32> = Box::new(1); // Create a `Box` pointing to the heap value 1.

    let a: i32 = *x; // Dereference `x` to get the value it points to, so a = 1.
    *x += 1; // Dereference `x` and increment the heap value. Now `x` points to 2.

    let r1: &Box<i32> = &x; // Borrow the `Box` itself (a reference to `x` on the stack).
    let b: i32 = **r1; // Double dereference: first `r1` to access `x`, then `*x` to access the heap value.

    let r2: &i32 = &*x; // Dereference `x` to get the heap value, then borrow it as `&i32`.
    let c: i32 = *r2; // Dereference `r2` (a reference to the heap value) to access the value directly.
}
// Context: Rust’s dereferencing and borrowing mechanics, specifically with smart pointers like Box<T>. The goal is to learn how to dereference smart pointers correctly and manage references to both the stack and heap. Pay attention to how the * operator is used to access and modify the values stored in the Box.

// Solution: Simplified code using implicit referencing.
fn borrowing_3() {
    let x: Box<i32> = Box::new(-1);
    let x_abs = x.abs(); // The `abs` method is called directly on `x` with implicit dereferencing.

    let r: &Box<i32> = &x;
    let r_abs = r.abs(); // The `abs` method works with implicit dereferencing through `r`.

    let s = String::from("Hello");
    let s_len = s.len(); // The `len` method is called directly on `s` without manually borrowing it.
}
// Context: Rust allows you to call methods on references without explicitly dereferencing them, thanks to the compiler's ability to automatically insert dereferences when needed.

// Solution: This version fixes the borrowing and mutation issue by rearranging the operations.
fn borrowing_4() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4); // Mutate the vector before borrowing any reference.
    let num: &i32 = &v[2]; // Borrow an immutable reference after mutation is complete.
    println!("Third element is {}", *num); // Now `num` is valid and safe to use.
}
// Context: The code attempted to borrow an immutable reference to an element in a vector while also mutating the vector. This is not allowed because mutation can invalidate references, potentially causing undefined behaviour.

// Solution: The code works as intended, demonstrating mutable references.
fn borrowing_5() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // Borrow a mutable reference to the third element.
    *num += 1; // Dereference `num` and increment the value it points to.
    println!("Third element is {}", *num); // Print the updated value (should be 4).
    println!("Vector is now {:?}", v); // Print the entire vector (should be [1, 2, 4]).
}
// Context: mutable references allow you to modify values directly through references. The current code uses a mutable reference to update an element in a vector, then safely prints the modified value and the entire vector.

// Solution: The code is fixed by correctly downgrading the mutable reference to an immutable one.
fn borrowing_6() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // Mutable reference to the third element.
    let num2: &i32 = &*num; // Downgrade the mutable reference to an immutable one for read-only access.
    println!("{} {}", *num, *num2); // Print both the mutable and downgraded immutable references.
}
// Context: downgrading mutable references in Rust allows you to temporarily create an immutable reference from a mutable reference for read-only purposes, as long as no conflicting mutable operations are performed during its usage.

// Solution: The code is fixed by ensuring the reference `y` is no longer used after `x` is modified.
fn lifetime_1() {
    let mut x = 1;
    let y = &x; // Start of the immutable borrow.
    let z = *y; // Dereference `y` to get the value of `x`.
                // `y` is no longer used after this point.
    x += z; // Safe to modify `x` now because the borrow has ended.
}
// Context: References in Rust are valid only within their appropriate scope.

fn main() {
    // Execute the function to see if your changes worked!
    borrowing_1();
    borrowing_2();
    borrowing_3();
    borrowing_4();
    borrowing_5();
    borrowing_6();

    lifetime_1();
}
