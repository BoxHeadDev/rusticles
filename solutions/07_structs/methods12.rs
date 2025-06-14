// Solution: mutable pointer is more idiomatic (doesn't take ownership)
struct Point(i32, i32);

impl Point {
    // Refactor to use a mutable reference.
    fn incr_v1(&mut self) {
        self.0 += 1;
    }
}

fn main() {
    let mut p = Point(0, 0);

    // Call the refactored method, which now uses a mutable reference.
    p.incr_v1();
    println!("{}", p.0); // Outputs: 1
}
// Context: In Rust, methods that modify the state of a struct should use mutable references (&mut self) instead of taking ownership (self). This allows the caller to retain ownership of the struct while enabling in-place modifications.
