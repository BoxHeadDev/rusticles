// 🦀 Rustlings Challenge: Mutable References with Lifetimes
//
// This function returns a mutable reference to the longer of two mutable `String`s.
// You'll need to:
// ✅ Add the correct lifetime annotations
// ✅ Use the correct `&'a mut T` syntax
//
// ⚠️ Don't try to use `'static` or clone anything — we want to work purely with references.

fn longer_string(x: &mut String, y: &mut String) -> &mut String {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let mut a = String::from("short");
    let mut b = String::from("much longer");

    let result = longer_string(&mut a, &mut b);
    result.push_str(" is longest!");
    println!("{result}");
}
