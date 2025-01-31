// Fix this function to avoid returning a reference to a local variable!
// The goal is to return a valid `String` or a valid reference.
fn return_a_string() -> &String {
    let s = String::from("Hello world"); // `s` is created as a local variable.
    &s // ERROR: Returning a reference to `s`, which will be deallocated after the function ends.
}

// Fix the code to respect Rust's borrowing rules and avoid ownership issues.
fn ownership_error1() {
    let name = vec![String::from("Ferris")];
    let first = &name[0]; // Immutable borrow of the first element.
    stringify_name_with_title(&name); // Attempt to modify `name` here.
    println!("{}", first); // ERROR: Immutable reference is still in use here!
}

// Ideally, the function should create a full name like:
// ["Ferris", "Jr."] => "Ferris Jr. Esq."
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq.")); // ERROR: Cannot mutate `name` while borrowed immutably.
    let full = name.join(" "); // Join the vector elements into a string.
    full
}

// Fix the code to avoid conflicting borrows while mutating the `dst` vector.
// The goal is to ensure no immutable references are active while the vector is being mutated.
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // Immutable borrow of `dst`.
    for s in src {
        if s.len() > largest.len() {
            // ERROR: `dst` cannot be mutated while `largest` exists.
            dst.push(s.clone()); // Mutating `dst` here conflicts with the immutable borrow.
        }
    }
}

// Fix the code to avoid ownership and double-free errors.
// The goal is to use the reference without transferring ownership.
fn ownership_error2() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0]; // Immutable reference to the first element of the vector.
    let s: String = *s_ref; // ERROR: Cannot take ownership of the value through a reference.
}

// Fix the code to respect Rust's borrowing rules around mutable references.
// The goal is to ensure there are no conflicting or invalid references.
fn ownership_error3() {
    let mut n = 0;
    let a = &mut n; // Create a mutable reference to `n`.
    let b = a; // ERROR: This transfers ownership of the mutable reference from `a` to `b`.
}

// Fix the code to ensure borrowing rules are respected.
// The goal is to safely borrow and mutate different parts of a tuple.
fn ownership_error4() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = get_first(&name); // Immutable reference to `name.0`.
    name.1.push_str(", Esq."); // ERROR: Cannot mutate `name` while `first` exists.
    println!("{first} {}", name.1);
}

fn get_first(name: &(String, String)) -> &String {
    &name.0 // Borrow the first element of the tuple.
}

// Fix the code to resolve conflicting borrows in the array.
// The goal is to safely access and modify different elements of the array.
fn ownership_error5() {
    let a = [0, 1, 2, 3];
    let x = &mut a[1]; // Mutable borrow of the second element.
    let y = &a[2]; // ERROR: Immutable borrow of the third element conflicts with the mutable borrow.
    *x += *y; // Attempt to modify the second element using the immutable third element.
}

// Fix the code to avoid double free errors while working with string ownership.
fn ownership_error6() {
    let s = String::from("Hello world"); // `s` owns the string.
    let s_ref = &s; // Borrow `s` immutably.
    let s2 = *s_ref; // ERROR: This moves the ownership of `s` from `s_ref` to `s2`.
    println!("{s2}"); // `s` is no longer accessible here, but we still try to use it.
}

// Fix the code to avoid borrowing conflicts when indexing the vector.
// The goal is to safely copy one element of the vector into another without violating borrowing rules.
fn ownership_error7() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1); // Copy the value of `v[0]` into `v[1]`.
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i]; // Mutable borrow of `v[i]`.
    *n = v[i - 1]; // ERROR: Immutable borrow of `v[i - 1]` conflicts with the mutable borrow of `v[i]`.
}

// Fix the code to respect borrowing rules and ensure safe usage of the string.
// The goal is to properly handle the `name` string without transferring ownership.
fn ownership_error8() {
    let name = String::from("Ferris");
    let name_ref = &name; // Immutable borrow of `name`.
    award_phd(&name); // Attempt to modify `name` by taking ownership.
    println!("{}", name_ref); // ERROR: `name_ref` might be invalidated.
}

fn award_phd(name: &String) {
    let mut name = *name; // ERROR: Takes ownership of `name`, causing conflicts.
    name.push_str(", Ph.D."); // Attempt to modify `name`.
}

// Fix the code to avoid conflicts with mutable references and ensure safe access to `point`.
// The goal is to safely modify and print the elements of the array.
fn ownership_error9() {
    let mut point = [0, 1];
    let mut x = point[0]; // Immutable copy of `point[0]`.
    let y = &mut point[1]; // Mutable borrow of `point[1]`.
    x += 1; // Increment the copy of `point[0]`.
    *y += 1; // ERROR: Cannot have a mutable borrow while `point` is still accessed.
    println!("{} {}", point[0], point[1]); // ERROR: Conflicting access to `point`.
}

fn main() {
    // Execute the function to see if your changes worked!
    let result = return_a_string();
    println!("{}", result);

    ownership_error1();

    let mut dst = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];

    let src = vec![
        String::from("kiwi"),
        String::from("pineapple"),
        String::from("grape"),
    ];

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
