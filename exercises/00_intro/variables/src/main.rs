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

fn variables(){
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn variables_2(){
  let x = 1;
  println!("{x}");
  x += 1;
  println!("{x}");
}

fn constants_1(){
    const THREE_HOURS_IN_SECONDS = 60 * 60 * 3;
    println!(THREE_HOURS_IN_SECONDS)
}

fn constants_2(){
    const mut TOTAL = 50;
    println!(TOTAL);
    TOTAL = 100;
    println!(TOTAL);
}

const THREE: u32 = 1 + 2;
fn constants_3() {
    println!("{THREE}");
}

fn shadowing_1(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn shadowing_2(){
    let mut spaces = "   ";
    spaces = spaces.len();
}

fn shadowing_3(){
  let mut x: u32 = 1;
  {
    let mut x = x;
    x += 2;
  }
  println!("{x}");
}

fn shadowing_4(){
  let mut x: u32 = 1;
  x = "Hello world";
  println!("{x}");
}
