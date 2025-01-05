fn main() {
    // Will it compile? Why?
    // What will be the output?
    ownership_1();
    ownership_2();
    ownership_3();
    ownership_4();
    ownership_5();
    ownership_6();
    ownership_7();
    ownership_8();
    ownership_9();
    ownership_10(); 
    ownership_11();
}

// Variable used before defined
fn ownership_1(){
    read(x);
    let x = true;
}

//
fn ownership_2(){
    let a_num = 4;
    make_and_drop();
}

// Ownership is moved to b
fn ownership_3(){
    let a = Box::new([0; 1_000_000]);
    let b = a;
}

// Using first when it no longer points to heap
fn ownership_4(){
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}, originally {first}");
}

// Clone creates new value on heap
fn ownership_5(){
    let second = String::from("Ferris");
    let second_clone = second.clone();
    let whole = add_suffix(second_clone);
    println!("{whole}, originally {second}");
}

// return value is assigned to s2
fn ownership_6(){
    let s = String::from("hello");
    let s2 = add_suffix(s);
    println!("{}", s2); // hello world
}

// s could be assigned to s2 causing undefined behaviour
fn ownership_7(){
    let s = String::from("hello");
    let s2;
    let b = false;
    if b {
        s2 = s;
    }
    println!("{}", s);
}

// b no longer points to value on heap
fn ownership_8(){
    let b = Box::new(0);
    let b2 = b;
    move_a_box(b);
}

// b is not deallocated until move_a_box is called
fn ownership_9(){
    let b = Box::new(0);
    let b2 = b;
    println!("{}", b);
    move_a_box(b2); 
}

// b is passed to move_a_box
fn ownership_10(){
    let b = Box::new(0);
    move_a_box(b);
    let b2 = b;
}

// b is passed to move_a_box
fn ownership_11(){
   let b = Box::new(0);
   move_a_box(b);
   println!("{}", b);
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn make_and_drop() {
    let a_box = Box::new(5);
}

fn add_suffix(mut s: String) -> String {
  s.push_str(" world");
  s
}
