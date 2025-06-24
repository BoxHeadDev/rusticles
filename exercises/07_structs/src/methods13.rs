// Fix the mutable borrow error so the code compiles.
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // This method returns a mutable reference to `x`.
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}

fn main() {
    let mut p = Point { x: 1, y: 2 };

    let x = p.get_x(); // Mutable borrow of `p` for `x`
    *x += 1; // Update `x`
    println!("{} {}", *x, p.y); // Error: Cannot access `p.y` while `x` holds a mutable borrow.
}
