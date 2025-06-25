// Solution:
struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    // This method returns a mutable reference to `x`.
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}

fn main() {
    let mut p = Point2 { x: 1, y: 2 };

    {
        let x = p.get_x(); // Mutable borrow of `p` for `x`
        *x += 1; // Update `x`
    } // Mutable borrow of `p` ends here

    println!("{} {}", p.x, p.y); // Now you can safely access `p.y`
}
// Context: In Rust, you cannot borrow a struct mutably and then access it while the mutable borrow is still active. This is to prevent undefined behaviour caused by simultaneous access and mutation. To fix such errors, you need to ensure that the mutable borrow’s lifetime ends before accessing other parts of the struct.
