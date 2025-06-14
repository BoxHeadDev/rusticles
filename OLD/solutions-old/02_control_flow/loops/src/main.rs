// Solution: Use `break` to return a value from the loop
fn loop_return_values() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Return `counter * 2` and exit the loop
        }
    };

    println!("The result is {result}"); // Prints "The result is 20"
}
// Context: In Rust, you can return a value from a loop by using the break statement with an associated value. This value is then assigned to a variable that is bound to the result of the loop.

// Solution: Use a label to break out of the outer loop
fn inner_loop_break() {
    let mut count = 0;
    'counting_up: loop {
        // Label the outer loop as `counting_up`
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Break only the inner loop
            }
            if count == 2 {
                break 'counting_up; // Break out of the outer loop using the label
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // Prints: "End count = 2"
}
// Context: In Rust, breaking out of a nested loop requires a loop label. Loop labels allow you to specify which loop you want to break out of when multiple loops are involved.

// Solution: Implement a `while` loop to count down from 3 to 1
fn while_loop() {
    let mut number = 3;

    while number != 0 {
        // Continue looping as long as `number` is not 0
        println!("{number}!"); // Print the current value of `number`

        number -= 1; // Decrement `number` by 1 on each iteration
    }

    println!("LIFTOFF!!!"); // Print "LIFTOFF!!!" after the countdown is complete
}
// Context: In Rust, a while loop allows you to repeatedly execute a block of code as long as a specified condition evaluates to true.

// Solution: Use a `while` loop to iterate through the array
fn collection_while_loop() {
    let a = [10, 20, 30, 40, 50]; // Define the array
    let mut index = 0; // Initialize the index variable

    while index < 5 {
        // Continue looping as long as the index is less than 5
        println!("the value is: {}", a[index]); // Print the value at the current index

        index += 1; // Increment the index to move to the next element
    }
}
// Context: In Rust, you can use a while loop to iterate through a collection, such as an array, by incrementing an index variable.

// Solution: Use a `for` loop to iterate through the array
fn collection_for_loop() {
    let a = [10, 20, 30, 40, 50]; // Define the array

    for element in a {
        // Iterate through each element in the array
        println!("the value is: {element}"); // Print the current element
    }
}

// Context: In Rust, you can use a for loop to iterate through elements of a collection directly, without the need for an index variable. This makes iterating simpler and less error-prone.

// Solution: Use a `for` loop with a reversed range
fn range_loop() {
    for number in (1..=4).rev() {
        // Use `1..=4` to include 4 and reverse the range
        println!("{number}!"); // Print each number in the countdown
    }

    println!("LIFTOFF!!!"); // Print "LIFTOFF!!!" after the countdown
}
// Context: In Rust, the for loop can iterate over ranges, and the rev() method can be used to reverse the range for descending iteration.

fn main() {
    // Execute the function to see if your changes worked!
    loop_return_values();
    inner_loop_break();
    while_loop();
    collection_while_loop();
    collection_for_loop();
    range_loop();
}
