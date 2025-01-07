fn main() {
    loop_return_values();
    inner_loop_break();
    while_loop();
    collection_while_loop();
    collection_for_loop();
}

// Assign the value to result when counter is 10
fn loop_return_values() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            counter * 2;
        }
    };

    println!("The result is {result}");
}

// Update to break out of all loops when count is 2
fn inner_loop_break() {
    let mut count = 0;
    loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// Implement a while loop that counts down from 3
fn while_loop() {
    let mut number = 3;

    println!("{number}!");

    println!("LIFTOFF!!!");
}

// Loop through the collection using a while loop
fn collection_while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("the value is: {}", a[index]);
}

// Loop through the collection using a for loop
fn collection_for_loop() {
    let a = [10, 20, 30, 40, 50];

    println!("the value is: {}");
}

// Loop from 4 to 1 using a range
fn range_loop() {
    println!("{number}!");

    println!("LIFTOFF!!!");
}
