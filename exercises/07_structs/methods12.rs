// Refactor th `incr_v1` method to take a mutable reference instead of ownership.
struct Point(i32, i32);

impl Point {
    fn incr_v1(mut self) {
        self.0 += 1;
    }
}

fn main() {
    let mut p = Point(0, 0);

    // Fix this method call to work with the refactored method.
    p.incr_x();
    println!("{}", p.0);
}
