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
        // ✅ This closure moves `captured` into itself
        println!("Using fallback value");
        captured
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
        // ✅ Mutating the counter — valid for FnMut
        sort_count += 1;
        r.width
    });

    println!("Sorted {sort_count} times.");
    println!("{list:#?}");

    // 🛑 Part 3: This closure tries to move out a value — fix it!
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
        // ❌ This line moves `message` — which breaks because `sort_by_key` expects `FnMut`
        // log.push(message);

        // ✅ FIX: Clone or use a reference instead
        log.push(message.clone());

        r.width
    });

    println!("{list2:#?}");
    println!("Log: {log:?}");
}
