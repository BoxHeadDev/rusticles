use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// TODO: Update the following so that Pair<T> only implements the cmp_display method
// if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
