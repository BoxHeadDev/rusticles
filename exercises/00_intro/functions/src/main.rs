fn main() {
    // Will the following functions compile?
    // What will be the output?

    another_function();
    print_labeled_measurement();
    expressions();
    
    let x = plus_one(5);
    println!("The value of x is: {x}");

    println!("{}", f({
        let y = 1;
        y + 1
    }));
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value, unit_label) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions() {
    let x = (let y = 6);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) {
    x + 1;
}

fn f(x: i32) -> i32 { x + 1 }
