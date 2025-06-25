// Solution: This version fixes all issues by correctly using dereferencing and borrowing.
fn main() {
    let mut x: Box<i32> = Box::new(1); // Create a `Box` pointing to the heap value 1.

    let a: i32 = *x; // Dereference `x` to get the value it points to, so a = 1.
    *x += 1; // Dereference `x` and increment the heap value. Now `x` points to 2.

    let r1: &Box<i32> = &x; // Borrow the `Box` itself (a reference to `x` on the stack).
    let b: i32 = **r1; // Double dereference: first `r1` to access `x`, then `*x` to access the heap value.

    let r2: &i32 = &*x; // Dereference `x` to get the heap value, then borrow it as `&i32`.
    let c: i32 = *r2; // Dereference `r2` (a reference to the heap value) to access the value directly.
}
// Context: Rust’s dereferencing and borrowing mechanics, specifically with smart pointers like Box<T>. The goal is to learn how to dereference smart pointers correctly and manage references to both the stack and heap. Pay attention to how the * operator is used to access and modify the values stored in the Box.
