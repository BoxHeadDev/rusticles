// Solution: Explicitly specifying the type ensures the compiler knows what type to parse the string into. In this case, we use u32.
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
}
// Context: The Rust compiler needs to know the specific type of a variable when parsing a string into a number, as there are multiple numeric types in Rust (e.g., u32, i32, f64). Without an explicit type, the code will fail to compile.
