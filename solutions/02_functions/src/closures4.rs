// 🦀 Rustlings Challenge: Closure Fn Traits
//
// This exercise demonstrates how closures implement Fn, FnMut, or FnOnce,
// based on how they use values from their environment.
//
// You're given 3 parts to complete:
// 1. A closure using FnOnce (moving a value out)
// 2. A closure using FnMut (mutating a captured value)
// 3. Fixing a broken closure that tries to move a value in `sort_by_key`, which expects FnMut
//
// HINTS:
// - Use `unwrap_or_else` to test `FnOnce` behavior.
// - Use a counter to avoid moving values for `FnMut`.
// - Closures that move values can only be called once!

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // ✅ Part 1: FnOnce - move a value into a closure
    let optional_string: Option<String> = None;
    let captured = String::from("fallback");

    let result = optional_string.unwrap_or_else(|| {
        println!("Using fallback value");
        captured // ✅ `captured` is moved here
    });

    println!("Result: {result}");

    // ✅ Part 2: FnMut - mutate captured value multiple times
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_count = 0;

    list.sort_by_key(|r| {
        sort_count += 1; // ✅ mutates the counter, implements FnMut
        r.width
    });

    println!("Sorted {sort_count} times.");
    println!("{list:#?}");

    // ✅ Part 3: Fix closure that tries to move a value (should be FnMut)
    let mut list2 = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let message = String::from("Sorting...");
    let mut log: Vec<String> = vec![];

    list2.sort_by_key(|r| {
        // ✅ Cloning instead of moving preserves ability to call closure more than once
        log.push(message.clone());
        r.width
    });

    println!("{list2:#?}");
    println!("Log: {log:?}");
}
