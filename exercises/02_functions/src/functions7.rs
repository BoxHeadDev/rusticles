// Fix the function by adding the correct type annotations to the parameters
fn print_labeled_measurement(value, unit_label) {
    println!("The measurement is: {value}{unit_label}");
}

fn main(){
    // Execute the function to see if your changes worked!

    print_labeled_measurement(); // Should print: "The measurement is: 5m"
}
