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

// ✅ Implemented Drop trait for Resource
impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping Resource with name `{}`!", self.name);
    }
}

fn main() {
    let res1 = Resource {
        name: String::from("res1"),
    };
    let res2 = Resource {
        name: String::from("res2"),
    };

    println!("Created resources.");

    // ✅ Explicitly dropping res1 early using std::mem::drop
    drop(res1);

    println!("Continuing execution...");
    // res2 is automatically dropped here at the end of the scope
}
