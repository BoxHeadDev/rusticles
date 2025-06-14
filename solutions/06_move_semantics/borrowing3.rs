// Solution: Simplified code using implicit referencing.
fn main() {
    let x: Box<i32> = Box::new(-1);
    let x_abs = x.abs(); // The `abs` method is called directly on `x` with implicit dereferencing.

    let r: &Box<i32> = &x;
    let r_abs = r.abs(); // The `abs` method works with implicit dereferencing through `r`.

    let s = String::from("Hello");
    let s_len = s.len(); // The `len` method is called directly on `s` without manually borrowing it.
}
// Context: Rust allows you to call methods on references without explicitly dereferencing them, thanks to the compiler's ability to automatically insert dereferences when needed.
