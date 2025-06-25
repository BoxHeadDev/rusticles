// 🦀 Rustlings Challenge: Box on the Heap
//
// Boxes allow you to store data on the heap while keeping a pointer to it on the stack.
// In this challenge, you'll:
// - Create a box storing an integer
// - Print the value stored in the box
//
// HINTS:
// - Use `Box::new()` to allocate on the heap
// - You can access the value in a Box using `*box_var` (dereference)

fn main() {
    let boxed_num = Box::new(42); // ✅ Stored 42 on the heap

    println!("The value in the box is: {}", boxed_num); // ✅ Box<T> implements Display if T does
}
