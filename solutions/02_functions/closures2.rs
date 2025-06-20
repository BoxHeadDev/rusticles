// 🦀 Rustlings Challenge: Closure Type Inference
//
// In this exercise, you will work with closures both with and without type annotations.
// You'll see how Rust infers types based on the first use of a closure and then "locks"
// them in. Fix the code to compile and match the expected output.
//
// HINTS:
// - Type annotations can make things clearer, but are optional.
// - Type inference for closures means you cannot use the same closure with multiple types.

fn main() {
    fn add_one_fn(x: u32) -> u32 {
        x + 1
    }

    let add_one_closure_annotated = |x: u32| -> u32 { x + 1 };

    let add_one_inferred = |x| x + 1;

    println!("Function call: {}", add_one_fn(5));
    println!("Closure annotated call: {}", add_one_closure_annotated(5));
    println!("Closure inferred call: {}", add_one_inferred(5));

    // ✅ Fix: Use separate closures for different types due to type inference
    let string_identity = |x: String| x;
    let s = string_identity(String::from("hello"));
    println!("String identity: {}", s);

    let number_identity = |x: i32| x;
    let n = number_identity(10);
    println!("Number identity: {}", n);
}
