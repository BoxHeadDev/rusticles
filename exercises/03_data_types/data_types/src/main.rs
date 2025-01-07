use std::io;

fn main() {
    types_1();
    types_2();
    types_3();
    types_4();
    types_5();
}

// Fix
fn types_1(){
    let guess = "42".parse().expect("Not a number!");
}

// Add explicit type annotations
fn types_2() {
    let t = true;
    let f = false;
    let c = 'z';
}

fn types_3() {
    let max: u8 = 255;
    let total = max + 1;

    println!("Value: {}", total);
}

fn types_4() {
    let x = 2.0; // What type will be inferred?
    let y = 3.0; // Assign a type
}

fn types_5(){
    let x: fsize = 2.0;
    println!("{x}");
}

fn types_6() {
    let tup = (500, 6.4, 1); // Add Explicit type
    
    // Destructure tup values to seperate variables

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}

fn types_7(){
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x;

    let six_point_four = x;

    let one = x;
}

fn types_8(){
    let a = [1, 2, 3, 4, 5]; // Add Explicit type

    let b = [3,3,3,3,3]; // Use Shorthand

    let first = a;
    let second = a;
}


// What happens when value not in array is requested?
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
