use std::fmt::Display;

trait MyToString {
    fn to_my_string(&self) -> String;
}

// Blanket implementation
impl<T: Display> MyToString for T {
    fn to_my_string(&self) -> String {
        format!("{}", self)
    }
}

fn main() {
    let num = 42;
    let word = String::from("Rust");

    println!("{}", num.to_my_string());
    println!("{}", word.to_my_string());
}
