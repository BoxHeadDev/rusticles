// 🦀 Rustlings Challenge: Lifetime Annotation Syntax
//
// In this exercise, you’ll work with a function that compares two integer references
// and returns one of them. You'll need to correctly annotate the lifetimes.
//
// Remember:
// - `&i32` is a reference without a named lifetime
// - `&'a i32` is a reference with a lifetime named `'a`
// - `&'a mut i32` is a mutable reference with lifetime `'a`
//
// ✅ This version compiles and tells the compiler: the returned reference lives
// at least as long as both input references.

fn pick_one<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if *x > *y { x } else { y }
}

fn main() {
    let a = 10;
    let b = 20;

    let result = pick_one(&a, &b);
    println!("The bigger number is: {result}");
}
