// We define a generic struct Point<T> where T can be any type.
struct Point<T> {
    x: T,
    y: T,
}

// We implement methods for Point<T>. This block is generic over T,
// meaning it applies to Point<i32>, Point<f64>, Point<String>, etc.
impl<T> Point<T> {
    // This method returns a reference to the `x` field of the Point.
    fn x(&self) -> &T {
        // We use &self.x to return a reference, avoiding unnecessary copies or moves.
        &self.x
    }
}

fn main() {
    // We create a Point<i32> here, since both x and y are integers.
    let p = Point { x: 5, y: 10 };

    // This prints the x value of the point using the method we implemented.
    println!("p.x = {}", p.x());
}
