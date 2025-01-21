fn main() {
    // Why is the program unsafe?
    return_a_string();

    ownership_error1();

    add_big_strings();

    ownership_error2();

    ownership_error3();

    ownership_error4();

    ownership_error5();

    ownership_error6();

    ownership_error7();

    ownership_error8();

    ownership_error9();
}

fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}

// Mutating read only data
// name in helper function is an immutable reference
fn ownership_error1() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" "); // join copies data (clone)
    full.push_str(" Esq.");
    full
}

// Aliasing and Mutating data structure
// Get length only to shorten the lifetime of borrowing dst
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

// Double free error
fn ownership_error2() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    // let s: String = *s_ref; // avoid taking ownership of string
}

// two mutatble references a and b
fn ownership_error3() {
    let mut n = 0;
    let a = &mut n;
    let b = a;
}

// get_first removed write permissions
fn ownership_error4() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = &name.0; //
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}

// Mutating different array elements
// Rust doesn't know the value of the index
fn ownership_error5() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    let y = &a[2];
    *x += *y;
}

// String is freed twice
fn ownership_error6() {
    let s = String::from("Hello world");
    let s_ref = &s;
    let s2 = *s_ref;
    println!("{s2}");
    // free s and s2
}

// No undefined behaviour
// Rust doesn't know indexing reference different elements
fn ownership_error7() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}

fn ownership_error8() {
    let name = String::from("Ferris");
    let name_ref = &name;
    award_phd(&name); // takes ownership
    println!("{}", name_ref);
}

fn award_phd(name: &String) {
    let mut name = *name; // takes ownership of string
    name.push_str(", Ph.D.");
    // string is deallocated
}

fn ownership_error9() {
    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1; // doesn't affect point
    *y += 1;
    println!("{} {}", point[0], point[1]); // 0 2
}
