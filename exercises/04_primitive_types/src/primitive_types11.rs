// Correct the type annotation for `x` to fix the compiler error.
fn main() {
    let x: fsize = 2.0; // Hint: `fsize` is not a valid type. Replace it with a valid floating-point type.
    println!("{x}");
}
