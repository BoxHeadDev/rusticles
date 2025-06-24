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
    let r; // declare a reference in the outer scope

    {
        let x = 5;
        r = &x; // 🛑 ERROR: `x` does not live long enough
    }

    println!("r: {}", r); // 💥 This uses a reference to a dropped value
}
