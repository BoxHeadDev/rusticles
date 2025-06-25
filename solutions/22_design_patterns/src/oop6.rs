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

impl Action for Greeter {
    fn perform(&self) {
        println!("Hello there!");
    }
}

impl Action for Joker {
    fn perform(&self) {
        println!("Why so serious?");
    }
}

// ✅ Uses static dispatch — compiler knows exact type at compile time
fn run_static<T: Action>(actor: T) {
    actor.perform();
}

// ✅ Uses dynamic dispatch — method call resolved at runtime via vtable
fn run_dynamic(actor: Box<dyn Action>) {
    actor.perform();
}

fn main() {
    // Static dispatch: fast, allows inlining
    run_static(Greeter);
    run_static(Joker);

    // Dynamic dispatch: adds flexibility, incurs runtime cost
    run_dynamic(Box::new(Greeter));
    run_dynamic(Box::new(Joker));

    // You won't see a performance difference in this tiny example, but
    // in large systems, the dispatch model can matter.
}
