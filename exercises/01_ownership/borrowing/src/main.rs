fn main() {
    // Will it compile?
    borrowing_1();
    borrowing_2();
    borrowing_3(); // Update with implicit dereferences
    borrowing_4();
    borrowing_5();
    borrowing_6();

    // Where does the lifetime of y start and stop?
    lifetime_1();
}

// Pointers
fn borrowing_1() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

fn borrowing_2() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let b: i32 = **r1; // two dereferences get us to the heap value

    let r2: &i32 = &*x; // r2 points to the heap value directly
    let c: i32 = *r2; // so only one dereference is needed to read it
}

// Implicit referencing
fn borrowing_3() {
    let x: Box<i32> = Box::new(-1);
    let x_abs = x.abs();

    let r: &Box<i32> = &x;
    let r_abs = r.abs();

    let s = String::from("Hello");
    let s_len = s.len();
}

// v is aliased and mutated
// Push invalidates the reference
fn borrowing_4() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);
    println!("Third element is {}", *num);
}

// Mutable references
fn borrowing_5() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}

// Downgrade mutable reference
fn borrowing_6() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = &*num; // read only
    println!("{} {}", *num, *num2);
}

fn lifetime_1() {
    let mut x = 1;
    let y = &x; // Start
    let z = *y; // Stop
    x += z;
}
