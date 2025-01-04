fn main() {
    // Will the following compile?
    // What is the output?
    single_condition();
    multiple_conditions();
    inline_expression();

    loop_return_values();
    inner_loop_break();
    while_loop();
    collection_while_loop();
    collection_for_loop();
}

fn single_condition(){
    let number = 3;
    
    if number == 3 {
        println!("number was three");
    }
}

fn multiple_conditions(){
    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Condition needs to return same type
fn inline_expression() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// Break out of loop
fn loop_return_values() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Use label to break out of inner loop
fn inner_loop_break() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// Loop through the collection using a while loop
fn collection_while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn collection_for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn range_loop() {

    for number in (1..4).rev() {
        println!("{number}!");
    }
    
    println!("LIFTOFF!!!");
}
