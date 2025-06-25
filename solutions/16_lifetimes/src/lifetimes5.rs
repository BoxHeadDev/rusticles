// 🦀 Rustlings Challenge: Preventing Dangling References
//
// This program tries to store a reference to a value created in a smaller scope.
// Your task is to fix the issue by ensuring that no dangling references are created.
//
// ⚠️ Don't just slap 'static on the reference! That's not the fix here.
//
// HINT:
// - You can move the value or adjust scopes to ensure the reference outlives the value.

fn main() {
    let x = 5; // ✅ Move the value into the outer scope
    let r = &x; // ✅ Now `r` refers to a value that lives long enough

    println!("r: {}", r); // ✅ Safe reference to `x`
}

// NOTE: Alternatively, you could return a reference from a function and require a lifetime annotation
// but for this challenge the focus is on understanding scope and preventing dangling references directly.
