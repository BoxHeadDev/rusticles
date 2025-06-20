// 🦀 Rustlings Challenge: RefCell<T> and Interior Mutability
//
// This challenge introduces `RefCell<T>` and the interior mutability pattern.
// You’ll demonstrate how to mutate data through an immutable reference,
// and observe what happens if borrowing rules are violated.
//
// Tasks:
// 1. Replace the `Box` with `RefCell` in `CounterWrapper`.
// 2. Use `.borrow_mut()` to mutate the internal value.
// 3. Try to borrow it mutably AND immutably at the same time (uncomment to see panic).
//
// HINT: `RefCell<T>` is in `std::cell`.
// NOTE: You’ll get a *runtime panic* if you break the borrow rules.
//
// Expected output (until the panic line is uncommented):
//     counter = 0
//     incremented counter = 1

use std::cell::RefCell;

struct CounterWrapper {
    value: RefCell<i32>, // ✅ Changed Box<i32> to RefCell<i32>
}

fn main() {
    let counter = CounterWrapper {
        value: RefCell::new(0),
    };

    println!("counter = {}", counter.value.borrow()); // ✅ Borrow immutably

    *counter.value.borrow_mut() += 1; // ✅ Mutate while inside immutable context
    println!("incremented counter = {}", counter.value.borrow()); // ✅ Confirm mutation

    // ⚠️ Runtime panic if uncommented:
    // let borrow1 = counter.value.borrow();
    // let borrow2 = counter.value.borrow_mut(); // ❌ This causes a runtime panic due to overlapping borrows!
}
