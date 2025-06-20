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
    // ✅ A function with explicit type annotation
    fn add_one_fn(x: u32) -> u32 {
        x + 1
    }

    // ✅ A closure with full annotation
    let add_one_closure_annotated = |x: u32| -> u32 { x + 1 };

    // ✅ A closure relying on inference
    let add_one_inferred = |x| x + 1;

    println!("Function call: {}", add_one_fn(5));
    println!("Closure annotated call: {}", add_one_closure_annotated(5));
    println!("Closure inferred call: {}", add_one_inferred(5));

    // 🧪 Now let’s demonstrate how closure type inference works:
    let identity = |x| x;

    let s = identity(String::from("hello")); // ✅ first call: sets type to String
    println!("String identity: {}", s);

    let n = identity(10); // ❌ ERROR: mismatched types
    println!("Number identity: {}", n); // 🛑 This will not compile!
}
