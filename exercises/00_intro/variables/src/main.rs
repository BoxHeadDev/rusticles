fn main() {
    // Will the following compile?
    // What will be the output if it does?
    variables_1();
    variables_2();

    constants_1();
    constants_2();
    constants_3();

    shadowing_1();
    shadowing_2();
    shadowing_3();
}

// Mutation keyword must be used
fn variables(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// Mutation keyword must be used
fn variables_2(){
  let mut x = 1;
  println!("{x}");
  x += 1;
  println!("{x}");
}

// Constants type must be annotated
fn constants_1(){
    const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;
    println!(THREE_HOURS_IN_SECONDS)
}

// Constants cannot be mutated
fn constants_2(){
    const TOTAL:u32 = 50;
    println!(TOTAL);
    const NEW_TOTAL:u32 = 100;
    println!(NEW_TOTAL);
}

// Constants can be global
const THREE: u32 = 1 + 2;
fn constants_3() {
    println!("{THREE}"); // 3
}

// Outer scope is not affected due to shadowing
fn shadowing_1(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6
}

// Use shadowing to create a new variable with the same name
fn shadowing_2(){
    let spaces = "   ";
    let spaces = spaces.len();
}

// Mutation does not affect the outer scope
fn shadowing_3(){
  let mut x: u32 = 1;
  {
    let mut x = x;
    x += 2;
  }
  println!("{x}"); // 1 
}

// A variable cannot be assigned to a value of a different type
fn shadowing_4(){
  let mut x: u32 = 1;
  x = 2;
  println!("{x}");
}
