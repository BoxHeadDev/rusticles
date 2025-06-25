// Simplify this code by removing explicit dereferencing where possible.
// Use method calls directly on the variables without manually dereferencing them.
fn main() {
    let x: Box<i32> = Box::new(-1);
    let x_abs = i32::abs(*x); // Simplify this method call.

    let r: &Box<i32> = &x;
    let r_abs = i32::abs(**r); // Simplify this method call.

    let s = String::from("Hello");
    let s_len = str::len(&s); // Simplify this method call.
}
