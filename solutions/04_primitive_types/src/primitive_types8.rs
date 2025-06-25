// Solution: Adding explicit type annotations makes the variable types clear and helps the compiler catch potential type-related errors.
fn main() {
    let t: bool = true; // `bool` is the type for boolean values.
    let f: bool = false; // Explicitly specifying `bool` for false.
    let c: char = 'z'; // `char` is the type for a single Unicode character.
}
// Context: In Rust, the compiler can often infer the types of variables, but there are times when adding explicit type annotations improves code clarity and prevents ambiguity. Explicit annotations make it easier for others (and the compiler) to understand the type of each variable, especially in more complex scenarios.
