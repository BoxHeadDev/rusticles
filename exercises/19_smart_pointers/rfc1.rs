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
    value: RefCell<i32>,
}

fn main() {
    let counter = CounterWrapper {
        value: RefCell::new(0),
    };

    println!("counter = {}", counter.value.borrow());

    // ✅ Mutate using interior mutability
    *counter.value.borrow_mut() += 1;
    println!("incremented counter = {}", counter.value.borrow());

    // ❌ Uncommenting the line below causes a panic at runtime due to overlapping borrows
    // let borrow1 = counter.value.borrow();
    // let borrow2 = counter.value.borrow_mut(); // ❗ Panics here!
    // println!("This will not print: {}, {}", borrow1, borrow2);
}
