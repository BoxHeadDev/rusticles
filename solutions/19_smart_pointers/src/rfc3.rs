// 🦀 Rustlings Challenge: Runtime Borrow Checking with RefCell<T>
//
// `RefCell<T>` tracks borrows at *runtime*, not compile time.
// You can have multiple immutable borrows or a single mutable borrow—but not both at once.
//
// Your job:
// 1. Use `.borrow_mut()` twice at the same time — this will cause a panic!
// 2. Comment out the second borrow to make the test pass.
// 3. Observe the difference between compile-time vs. runtime checking.
//
// Expected panic message (when uncommented):
//   thread 'main' panicked at 'already borrowed: BorrowMutError'

use std::cell::RefCell;

fn main() {
    let messages = RefCell::new(vec![]);

    let mut first = messages.borrow_mut(); // ✅ first mutable borrow is valid
    // let mut second = messages.borrow_mut(); // ❌ This causes a panic at runtime!

    first.push("first message");

    // ✅ second borrow happens after first goes out of scope
    drop(first); // 👈 Explicitly drop `first` to end the mutable borrow

    let snapshot = messages.borrow(); // ✅ Now safe: we only have an immutable borrow
    println!("Messages: {:?}", snapshot); // ✅ Output: ["first message"]
}
