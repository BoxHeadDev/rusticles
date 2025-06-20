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
// HINT: Think about when a pattern *must* match (e.g. regular `let`) vs. when it *might not* (e.g. conditional control flow).

fn main() {
    let maybe_number = Some(42);

    // ❌ ERROR: `Some(x)` is refutable — `let` alone can't handle a `None` case.
    // let Some(x) = maybe_number;

    // TODO: Fix this using `if let` or `let ... else`
    // ✅ Print: "The number is 42" if it exists
    // ✅ Otherwise, print: "No number found"

    // ❌ WARNING: This pattern is irrefutable and always matches; `if let` is unnecessary.
    // let x = 10 else {
    //     println!("This will never run!");
    //     return;
    // };

    // TODO: Fix the above by simplifying the pattern to use just `let`.

    // ✅ This `match` uses a refutable pattern correctly
    match maybe_number {
        Some(n) => println!("Matched number: {}", n),
        None => println!("Matched nothing."),
    }
}
