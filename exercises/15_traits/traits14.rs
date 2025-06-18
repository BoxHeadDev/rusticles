// This exercise is about simplifying function signatures using a `where` clause
// for readability when dealing with complex trait bounds.

// Fix the function signature using a `where` clause instead of writing the bounds inline.

use std::fmt::{Debug, Display};

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    println!("t: {}, u: {:?}", t, u);
    42
}

fn main() {
    let string_val = String::from("hello");
    let vec_val = vec![1, 2, 3];

    some_function(&string_val, &vec_val);
}
