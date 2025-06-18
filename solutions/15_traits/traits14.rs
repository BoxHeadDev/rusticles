// This exercise is about simplifying function signatures using a `where` clause
// for readability when dealing with complex trait bounds.

// Fix the function signature using a `where` clause instead of writing the bounds inline.

use std::fmt::{Debug, Display};

// Replace the line above with a `where` clause version that compiles.
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("t: {}, u: {:?}", t, u);
    42
}

fn main() {
    let string_val = String::from("hello");
    let vec_val = vec![1, 2, 3];

    some_function(&string_val, &vec_val);
}
