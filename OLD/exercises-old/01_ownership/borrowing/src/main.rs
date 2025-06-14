// Fix this code to make it compile!
// The goal is to use borrowing to avoid ownership errors.
fn borrowing_1() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2); // ERROR: Ownership issues here!
    let s = format!("{} {}", m1, m2); // ERROR: m1 and m2 are no longer accessible!
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}

// Fix this code to make it compile and behave as intended!
// The goal is to properly dereference and borrow the values stored in a `Box`.
fn borrowing_2() {
    let mut x: Box<i32> = Box::new(1);

    let a: i32 = x; // ERROR: Cannot move value out of a `Box`.
    x += 1; // ERROR: Need to modify the value inside the `Box`.

    let r1: &Box<i32> = x; // ERROR: Cannot directly borrow `x` like this.
    let b: i32 = r1; // ERROR: Dereferencing is required here.

    let r2: &i32 = x; // ERROR: Need to borrow the value stored inside the `Box`.
    let c: i32 = r2; // ERROR: Missing dereference to access the value.
}

// Simplify this code by removing explicit dereferencing where possible.
// Use method calls directly on the variables without manually dereferencing them.
fn borrowing_3() {
    let x: Box<i32> = Box::new(-1);
    let x_abs = i32::abs(*x); // Simplify this method call.

    let r: &Box<i32> = &x;
    let r_abs = i32::abs(**r); // Simplify this method call.

    let s = String::from("Hello");
    let s_len = str::len(&s); // Simplify this method call.
}

// Fix this code to make it compile by respecting Rust's borrowing rules.
// The goal is to handle borrowing and mutation correctly.
fn borrowing_4() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2]; // ERROR: Immutable borrow while `v` might be mutated.
    v.push(4); // ERROR: Cannot mutate `v` while an immutable reference exists.
    println!("Third element is {}", *num); // ERROR: `num` may have been invalidated.
}

// Complete the code to ensure it works correctly with mutable references.
// The goal is to modify the third element of the vector using a mutable reference.
fn borrowing_5() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // Borrow a mutable reference to the third element.
    *num += 1; // Modify the value through the mutable reference.
    println!("Third element is {}", *num); // Print the updated value.
    println!("Vector is now {:?}", v); // Print the entire vector.
}

// Fix the code to handle a mutable reference and a downgraded immutable reference.
// Ensure the code compiles and runs correctly without violating borrowing rules.
fn borrowing_6() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // Mutable reference to the third element.
    let num2: &i32 = &*num; // ERROR: Properly downgrade the mutable reference to an immutable one.
    println!("{} {}", *num, *num2); // Print both references.
}

// Where does the lifetime of y start and stop?
fn lifetime_1() {
    let mut x = 1;
    let y = &x; // Borrow `x` immutably.
    let z = *y; // Dereference `y` to get the value of `x`.
    x += z; // ERROR: `x` is modified while `y` still exists. Fix the lifetimes!
}

fn main() {
    // Execute the function to see if your changes worked!
    borrowing_1();
    borrowing_2();
    borrowing_3();
    borrowing_4();
    borrowing_5();
    borrowing_6();

    lifetime_1();
}
