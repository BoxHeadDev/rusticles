// 🦀 Rustlings Challenge: Define a Smart Pointer
//
// You are creating a custom smart pointer `MyBox<T>` that wraps a value.
// You can use `.0` to access the inner value, but `*mybox` won’t work — yet.
//
// This exercise shows that without implementing `Deref`, Rust won’t let you use `*mybox`.
//
// HINTS:
// - Try to compile this code and read the error
// - You'll fix this in the next challenge by implementing `Deref`

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 42;
    let y = MyBox::new(x);

    assert_eq!(42, x);
    // 🔴 This will not compile! `MyBox<T>` does not implement `Deref`
    assert_eq!(42, *y);
}
