// Solution: floating-point types are infered and defined
fn main() {
    let x = 2.0; // By default, Rust infers `f64` for floating-point literals.
    let y: f32 = 3.0; // Explicitly specify `f32` to use a 32-bit floating-point type.
}
// Context: Rust provides two floating-point types: f32 and f64. By default, Rust infers floating-point numbers as f64 for better precision. However, in some cases, you might want to use f32 explicitly to optimise for memory usage or performance.
