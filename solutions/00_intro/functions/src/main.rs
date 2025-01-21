fn main() {
    // Will the following functions compile?
    // What will be the output?

    another_function(); // Output: Another Function
    print_labeled_measurement(5, 'h'); // Arguments needed
    expressions();
    
    let x = plus_one(5);
    println!("The value of x is: {x}");

    println!("{}", f({
        let y = 1;
        y + 1
    })); // Output: 3
}

fn another_function() {
    println!("Another function.");
}

// Parameter types
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements don't return values
fn expressions() {
    let x = let y = 6;
    println!("The value of x is: {x}");
}

// Return type
fn plus_one(x: i32) -> i32 {
    x + 1;
}

fn f(x: i32) -> i32 { x + 1 }
