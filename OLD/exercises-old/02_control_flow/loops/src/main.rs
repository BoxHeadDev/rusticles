// Fix the code to correctly return a value from the loop
fn loop_return_values() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            counter * 2; // This does not return a value. How can you fix it?
        }
    };

    println!("The result is {result}"); // Should print the result
}

// Fix the code to break out of all loops when `count` is 2
fn inner_loop_break() {
    let mut count = 0;
    loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Breaks only the inner loop
            }
            if count == 2 {
                break; // How can you break out of all loops here?
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // Should print: "End count = 2"
}

// Complete the code by implementing a `while` loop that counts down from 3
fn while_loop() {
    let mut number = 3;

    // Add a `while` loop here to count down from 3 to 1
    println!("{number}!");

    println!("LIFTOFF!!!"); // This should print after the countdown is complete
}

// Complete the code to loop through the collection using a `while` loop
fn collection_while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Add a `while` loop here to iterate through the array `a`
    println!("the value is: {}", a[index]); // This should print each value in the array
}

// Complete the code to loop through the collection using a `for` loop
fn collection_for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// Complete the code to loop through a range so that there is a countdown from 4 to 1
fn range_loop() {
    println!("{number}!");

    println!("LIFTOFF!!!");
}

fn main() {
    // Execute the function to see if your changes worked!
    loop_return_values();
    inner_loop_break();
    while_loop();
    collection_while_loop();
    collection_for_loop();
    range_loop();
}
