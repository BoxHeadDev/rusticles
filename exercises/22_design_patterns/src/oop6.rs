// 🦀 Rustlings Challenge: Trait Objects Perform Dynamic Dispatch
//
// This exercise compares static dispatch (generics) and dynamic dispatch (trait objects).
//
// You will:
// - Implement the `Action` trait for two types: `Greeter` and `Joker`.
// - Create a function `run_static` that takes a generic type implementing `Action` (static dispatch).
// - Create a function `run_dynamic` that takes a trait object `Box<dyn Action>` (dynamic dispatch).
//
// Pay attention to where dynamic dispatch occurs and where monomorphization allows inlining.
//
// 💡 HINT: Generics use static dispatch. Trait objects use dynamic dispatch via vtables.

trait Action {
    fn perform(&self);
}

struct Greeter;
struct Joker;

// TODO: Implement `Action` for both Greeter and Joker

// TODO: This uses static dispatch
fn run_static<T: Action>(actor: T) {
    actor.perform();
}

// TODO: This uses dynamic dispatch
fn run_dynamic(actor: Box<dyn Action>) {
    actor.perform();
}

fn main() {
    // ✅ This uses static dispatch
    run_static(Greeter);
    run_static(Joker);

    // ✅ This uses dynamic dispatch
    run_dynamic(Box::new(Greeter));
    run_dynamic(Box::new(Joker));

    // Observe how both static and dynamic dispatch call the same method but through different paths.
}
