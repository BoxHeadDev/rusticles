use std::slice::Iter;

fn vectors_1() {
    // Create an empty vector of integers.
    let mut v1: Vec<i32> = ??; // Hint: Use `Vec::new()`.

    // Add integers to the vector (e.g., 5, 6, 7, 8).
    ??; // Hint: Use the `.push()` method.
}

fn vectors_2() {
    // Create a populated vector of integers.
    let v2 = ??; // Hint: Use the `vec!` macro.

    // Assign the third element in the vector using indexing.
    let third = ??; // Hint: Use indexing (e.g., `v2[2]`).
    println!("The third element is {third}");

    // Assign the third element in the vector using the `.get()` method.
    let third = ??; // Hint: Use `v2.get(2)` to retrieve an `Option`.
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn vectors_3() {
    // What do each of the following return below?
    let v = vec![1, 2, 3, 4, 5];

    // Access an out-of-bounds index using indexing.
    let does_not_exist = &v[100]; // What happens here?

    // Access an out-of-bounds index using `.get()`.
    let does_not_exist = v.get(100); // What does this return?
}

fn vectors_4() {
    // What is wrong with the following code?
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // Immutable reference to the first element.
    v.push(6);         // Mutable borrow to add an element to the vector.

    println!("The first element is: {first}");
}

fn vectors_5() {
    // Loop over v and print each element after adding 1.
    let v = vec![100, 32, 57];

    for n_ref in &v {
        let n_plus_one: i32 = ??; // Hint: Dereference `n_ref` to access its value.
        println!("{n_plus_one}");
    }
}

fn vectors_6() {
    // Loop over v and add 50 to each element.
    let mut v = vec![100, 32, 57];

    for n_ref in &mut v {
        ??; // Hint: Use `*n_ref` to access and modify the value.
    }
}

fn vectors_7() {
    // What is wrong with the following code?
    let v = vec![String::from("Hello ")];

    let mut s = v[0]; // Error: Cannot move out of a vector by indexing.
    s.push_str("world");
    println!("{s}");
}

fn vectors_8() {
    // Fix the following? (Vectors can only have one type)
    let row = vec![3, String::from("blue"), 10.12]; // This doesn't work.
}

fn vectors_9() {
    // Does the following compile?
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        v.push(*i); // Mutating the vector while iterating over it.
    }
    println!("{} {} {}", v[3], v[4], v[5]);
}

fn vectors_10() {
    // What is the output?
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        v2.push(i); // Hint: Add a mutable reference to the vector.
    }

    *v2[0] = 5; // Hint: Modify the first element through the reference.
    let a = *v2[0];
    let b = v[0];

    println!("{a} {b}"); // What values will be printed?
}

fn iterator_1() {
    // What are the values of the following iterator?
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();

    let n1: &i32 = iter.next().unwrap(); // What value does this return?
    let n2: &i32 = iter.next().unwrap(); // What value does this return?
    let end: Option<&i32> = iter.next(); // What value does this return?
}

fn iterator_2() {
    // Iterate over a vector without using a pointer.
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: ? = ??; // Hint: Use a range of indices (0..v.len()).

    let i1: usize = iter.next().unwrap(); // Retrieve the first index.
    let n1: &i32 = &v[i1];               // Access the element at the index.
}

fn iterator_3() {
    // Why does the following not compile?
    fn dup_in_place(v: &mut Vec<i32>) {
        for n_ref in v.iter() { // Immutable borrow here.
            v.push(*n_ref); // Mutable borrow here, causing a conflict.
        }
    }
}

fn main() {
    // Execute the function to see if your changes worked!
    vectors_1();
    vectors_2();
    vectors_3();
    vectors_4();
    vectors_5();
    vectors_6();
    vectors_7();
    vectors_8();
    vectors_9();
    vectors_10();

    iterator_1();
    iterator_2();
    iterator_3();
}
