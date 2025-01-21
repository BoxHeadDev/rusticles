use std::io;

fn main() {
    // Will the following compile?
    // What will be the output?
    types_1();
    types_2();
    types_3();
    types_4();
    types_5();
    types_6();
    types_7();
    types_8();
    types_9();
}

fn types_1(){
    // Explicit type
    let guess:u32 = "42".parse().expect("Not a number!");
}

fn types_2() {
    let t: bool = true;
    let f: bool = false;
    let c: char = 'z';
}

fn types_3() {
    let max: u8 = 255; // Maximum value for u8
    let total = max.wrapping_add(1); // Adding 1 causes wrapping

    println!("Value: {}", total);
}

fn types_4() {
    let x = 2.0; // f64 default
    let y:f32 = 3.0;
}

// The type fsize does not exist.
// Floats must be either f32 or f64.
fn types_5(){
    let x: f32 = 2.0;
    println!("{x}");
}

fn types_6() {
    let tup: (i32, f32, u8) = (500, 6.4, 1);

    let (x,y,z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}

// Access value using index
fn types_7(){
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

fn types_8(){
    let a: [i32;5] = [1, 2, 3, 4, 5];

    let b = [3; 5];

    let first = a[0];
    let second = a[1];
}


// What happens when value not in array is requested?
// PANIC
fn types_9() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
