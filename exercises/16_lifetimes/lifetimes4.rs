// Where does the lifetime of y start and stop?
fn main() {
    let mut x = 1;
    let y = &x; // Borrow `x` immutably.
    let z = *y; // Dereference `y` to get the value of `x`.
    x += z; // ERROR: `x` is modified while `y` still exists. Fix the lifetimes!
}
