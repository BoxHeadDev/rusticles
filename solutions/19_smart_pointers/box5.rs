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

    let y = Box::new(x); // ✅ Allocated on the heap via Box

    assert_eq!(42, x);
    assert_eq!(42, *y); // ✅ Deref works just like a reference
}
