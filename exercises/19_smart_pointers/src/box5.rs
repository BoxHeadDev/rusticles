// 🦀 Rustlings Challenge: Box Dereferencing
//
// A Box<T> behaves like a reference when you dereference it using the `*` operator.
// This works because `Box<T>` implements the `Deref` trait.
//
// Your task:
// - Replace the reference-based code with Box<T>
// - Ensure the final assert works using `*box_val`
// - Do not use `&` or references at all

fn main() {
    let x = 42;

    // TODO: Replace the reference with a Box
    let y = &x; // ❌ Replace this with Box::new(x)

    assert_eq!(42, x);
    assert_eq!(42, *y); // ✅ This should still compile and pass!
}
