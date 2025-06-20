// 🦀 Rustlings Challenge: The Borrow Checker and Lifetimes
//
// The Rust compiler uses a borrow checker to make sure that all references are valid.
// In this exercise, you’ll fix a program that attempts to use a reference
// to a value that has gone out of scope.
//
// HINTS:
// - Pay attention to how long each variable lives.
// - You may need to reorder or restructure the code to make the reference valid.

fn main() {
    let r; // ---------+-- 'a
    //          |
    {
        //          |
        let x = 42; // -+-- 'b  |
        r = &x; //  |       |
        //  | 🛑 `x` does not live long enough for `r`
    } // -+       |
    //          |
    println!("r: {}", r); // 💥 This would cause undefined behavior if allowed!
} // ---------+
