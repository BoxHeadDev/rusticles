// Fix the code so that it compiles and runs successfully!
// The goal is to understand how to implement methods on structs with generic types.

struct Point<T> {
    x: T,
    y: T,
}

// Note: this should work for any type T!
impl Point {
    fn x(&self) {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
