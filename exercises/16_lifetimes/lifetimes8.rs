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
// ❌ This function will not compile without lifetime annotations.
// 🧠 Your goal: describe the relationship between the input references and the returned reference.

fn pick_one(x: &i32, y: &i32) -> &i32 {
    if *x > *y { x } else { y }
}

fn main() {
    let a = 10;
    let b = 20;

    let result = pick_one(&a, &b);
    println!("The bigger number is: {result}");
}
