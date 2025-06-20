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
    // TODO: Store the value 42 on the heap using a box
    let boxed_num = Box::new(0); // ❌ This currently stores the wrong value

    println!("The value in the box is: {}", boxed_num);

    // ✅ This should print: "The value in the box is: 42"
}
