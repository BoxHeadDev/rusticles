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

fn borrowing_1() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}

fn borrowing_2() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = x;
    x += 1;

    let r1: &Box<i32> = x;
    let b: i32 = r1;

    let r2: &i32 = x;
    let c: i32 = r2;
}

fn borrowing_3() {
    let x: Box<i32> = Box::new(-1);
    let x_abs = i32::abs(*x);

    let r: &Box<i32> = &x;
    let r_abs = i32::abs(**r);

    let s = String::from("Hello");
    let s_len = str::len(&s);
}

fn borrowing_4() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);
    println!("Third element is {}", *num);
}

fn borrowing_5() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}

fn borrowing_6() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);
}

fn lifetime_1() {
    let mut x = 1;
    let y = &x;
    let z = *y;
    x += z;
}
