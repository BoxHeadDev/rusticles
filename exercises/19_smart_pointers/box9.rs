// 🦀 Rustlings Challenge: Drop Trait Behavior
//
// Implement the Drop trait for `Resource`, so it prints a message when dropped.
// Then, explicitly drop the first resource early using `std::mem::drop`.
//
// HINT: The `drop` method is *not* called manually — use the `drop` function instead.
// Remember: `Drop::drop()` cannot be called directly by your code.
//
// Expected output (order matters!):
//     Created resources.
//     Dropping Resource with name `res1`!
//     Continuing execution...
//     Dropping Resource with name `res2`!

struct Resource {
    name: String,
}

// TODO: Implement the Drop trait for Resource.
// Print: Dropping Resource with name `<name>`!

fn main() {
    let res1 = Resource {
        name: String::from("res1"),
    };
    let res2 = Resource {
        name: String::from("res2"),
    };

    println!("Created resources.");

    // TODO: Drop res1 early using std::mem::drop
    // NOTE: Do *not* call `res1.drop()` manually!

    println!("Continuing execution...");
}
