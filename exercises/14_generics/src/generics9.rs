// In this exercise, you'll complete a method that mixes values from two different Points.
// The `Point` struct has two generic type parameters, but the `mixup` method will introduce its own!

// Fix the code so it compiles and prints:
// p3.x = 5, p3.y = c

struct Point<X, Y> {
    x: X,
    y: Y,
}

// Implement a method `mixup` that takes another Point with potentially different types,
// and returns a new Point using `self.x` and `other.y`.
impl<X, Y> Point<X, Y> {
    fn mixup<X, Y>(self, other: Point<X, Y>) -> Point<_, _> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
