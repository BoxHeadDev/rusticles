// 🦀 Rustlings Challenge: Refutability in Patterns
//
// Refutable patterns can fail to match and must be handled with `if let` or `let ... else`.
// Irrefutable patterns always match and are required for `let`, `for`, and function parameters.
//
// In this exercise:
// - Fix the invalid usage of a refutable pattern with `let`.
// - Use `if let` or `let ... else` where appropriate.
// - Do not silence compiler warnings by ignoring them — fix the actual issue!
//

fn main() {
    let maybe_number = Some(42);

    // ✅ Fixed: Used `let ... else` to handle the refutable pattern explicitly
    let Some(x) = maybe_number else {
        println!("No number found");
        return;
    };
    println!("The number is {}", x);

    // ✅ Fixed: This is an irrefutable pattern, so we just use a simple `let`
    let x = 10;
    println!("x is {}", x);

    // ✅ This `match` uses a refutable pattern correctly
    match maybe_number {
        Some(n) => println!("Matched number: {}", n),
        None => println!("Matched nothing."),
    }
}
