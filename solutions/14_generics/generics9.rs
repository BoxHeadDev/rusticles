// A generic struct with two type parameters: X1 and Y1.
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// We're implementing methods for any Point<X1, Y1>.
impl<X1, Y1> Point<X1, Y1> {
    // `mixup` takes another Point<X2, Y2> and returns a Point<X1, Y2>.
    // This allows combining `self.x` with `other.y`, even if their types differ.
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // Point<i32, f64>
    let p1 = Point { x: 5, y: 10.4 };

    // Point<&str, char>
    let p2 = Point { x: "Hello", y: 'c' };

    // This results in Point<i32, char>
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
