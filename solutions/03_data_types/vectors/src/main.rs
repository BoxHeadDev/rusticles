use std::slice::Iter;

fn vectors_1() {
    // Create an empty vector of integers.
    let mut v1: Vec<i32> = Vec::new();

    // Add integers to the vector.
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    println!("{:?}", v1); // Outputs: [5, 6, 7, 8]
}
// Context: Vectors in Rust are dynamic arrays that can grow or shrink in size. They are one of the most common collection types in Rust, and they require the Vec<T> type to define the type of elements they hold.

fn vectors_2() {
    // Create a populated vector of integers.
    let v2 = vec![1, 2, 3];

    // Assign the third element in the vector using indexing.
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    // Assign the third element in the vector using the `.get()` method.
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
// Context: Vectors in Rust allow you to access elements in two primary ways:
// Using indexing (e.g., v[2]) to directly access an element at a specific position.
// Using the .get() method, which returns an Option to handle cases where the index might be out of bounds.

fn vectors_3() {
    // What do each of the following return below?
    let v = vec![1, 2, 3, 4, 5];

    // Accessing an out-of-bounds index using indexing causes a panic.
    // let does_not_exist = &v[100]; // Uncommenting this will panic at runtime.

    // Accessing an out-of-bounds index using `.get()` returns `None`.
    let does_not_exist = v.get(100); // Returns None.
    match does_not_exist {
        Some(value) => println!("Found value: {value}"),
        None => println!("Index out of bounds!"),
    }
}
// Context: Vectors in Rust provide two ways to access elements: indexing (v[100]) and the .get() method. Accessing an out-of-bounds index using indexing causes a runtime panic, while .get() safely returns None if the index does not exist. This ensures that you can handle errors gracefully when using .get().

fn vectors_4() {
    // Rust prevents mutable and immutable references from coexisting.
    let mut v = vec![1, 2, 3, 4, 5];

    // Resolve by limiting the scope of the immutable reference.
    let first = &v[0]; // Immutable reference to the first element.
    println!("The first element is: {first}");

    v.push(6); // Mutable borrow to add an element to the vector.
}
// Context: In Rust, you cannot have a mutable reference (&mut) and an immutable reference (&) to the same data in the same scope. This ensures memory safety and prevents data races. In this challenge, you’ll fix a code snippet that violates Rust’s borrowing rules when modifying a vector while holding an immutable reference.

fn vectors_5() {
    // Loop over v and print each element after adding 1.
    let v = vec![100, 32, 57];

    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1; // Dereference `n_ref` to access its value.
        println!("{n_plus_one}");
    }
}
// Context: In Rust, when iterating over a vector, you can borrow its elements using a reference to avoid consuming the vector. This allows the vector to remain available for further use after the iteration. To access the actual value of a borrowed element, you need to dereference it using the * operator.

fn vectors_6() {
    // Loop over v and add 50 to each element.
    let mut v = vec![100, 32, 57];

    for n_ref in &mut v {
        *n_ref += 50; // Dereference `n_ref` to modify the value.
    }

    println!("{:?}", v); // Outputs: [150, 82, 107]
}
// Context: In Rust, you can iterate over a vector and mutate its elements by borrowing them mutably. This allows you to modify the values in-place without creating a new vector. To change the value of a mutable reference, you must use the * operator to dereference it.

fn vectors_7() {
    let v = vec![String::from("Hello ")];

    // Solution 1: Fix by borrowing the value from the vector.
    let s = &v[0]; // Borrow the value immutably.
    let mut s = s.clone(); // Clone the borrowed value to make it mutable.

    // Solution 2: Fix by cloning the value directly.
    let mut s = v[0].clone(); // Clone the value to move it out safely.

    s.push_str("world");
    println!("{s}");
}
// Context: In Rust, elements of a vector are owned by the vector, and indexing directly (v[0]) tries to move the value out of the vector. This is not allowed for types like String, which are non-copyable. Instead, you must use .get() with references or explicitly clone the value if you want to work with it outside the vector.

fn vectors_8() {
    // Define an enum to handle different types of data in the vector.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Create a vector of `SpreadsheetCell` to store heterogeneous data.
    let row = vec![
        SpreadsheetCell::Int(3),                     // Store an integer.
        SpreadsheetCell::Text(String::from("blue")), // Store a string.
        SpreadsheetCell::Float(10.12),               // Store a float.
    ];
}
// Context: Rust vectors can only hold elements of a single type. If you need to store heterogeneous data (e.g., integers, floats, and strings) in the same vector, you can define an enum with variants for each type. By wrapping the values in the enum, you can use the vector to store different types in a type-safe manner.

// Error: This code does not compile because you cannot borrow v mutably to iterate over it (for i in &mut v) and simultaneously mutate v by adding elements (v.push(*i)).
// Solution: To fix this, you can use a separate vector to store new elements and then extend the original vector after the iteration is complete.
fn vectors_9() {
    let mut v = vec![1, 2, 3];
    let mut new_elements = vec![];

    // Iterate over the vector and prepare new elements to add later.
    for i in &v {
        new_elements.push(*i);
    }

    // Extend the original vector with the new elements.
    v.extend(new_elements);

    // Safely access the new elements.
    println!("{} {} {}", v[3], v[4], v[5]); // Outputs: 1 2 3
}
// Context: In Rust, you cannot mutate a collection (e.g., adding elements to a vector) while iterating over it. This is because iteration borrows the collection immutably or mutably, and mutating the collection during iteration would violate Rust’s borrowing rules, potentially leading to undefined behaviour.

fn vectors_10() {
    // What is the output?
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        v2.push(i); // Push mutable references to `v` into `v2`.
    }

    *v2[0] = 5; // Modify the first element of `v` through the mutable reference in `v2`.
    let a = *v2[0]; // Dereference the first mutable reference to get its value.
    let b = v[0]; // Access the first element of `v` directly.

    println!("{a} {b}"); // Outputs: 5 5
}
// Context: In Rust, you can create a vector of mutable references to the elements of another vector. When you modify an element through the mutable reference, the changes are reflected in the original vector. This demonstrates how Rust ensures memory safety while allowing powerful, low-level operations.

fn iterator_1() {
    // What are the values of the following iterator?
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();

    let n1: &i32 = iter.next().unwrap(); // Returns the first element: 1
    let n2: &i32 = iter.next().unwrap(); // Returns the second element: 2
    let end: Option<&i32> = iter.next(); // Returns None, as there are no more elements.

    println!("n1: {n1}, n2: {n2}, end: {:?}", end); // Outputs: n1: 1, n2: 2, end: None
}
// Context: In Rust, an iterator allows you to sequentially access elements in a collection without needing to manage indexing or bounds manually. Calling .next() on an iterator yields the next item in the collection or None if the iterator has reached the end. Iterators are a core concept in Rust, enabling functional programming patterns and efficient data manipulation.

fn iterator_2() {
    // Iterate over a vector without using a pointer.
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: std::ops::Range<usize> = 0..v.len(); // Create a range of indices.

    let i1: usize = iter.next().unwrap(); // Retrieve the first index (0).
    let n1: &i32 = &v[i1]; // Access the element at the index.

    println!("Index: {i1}, Value: {n1}"); // Outputs: Index: 0, Value: 1
}
// Context: Instead of using .iter() to borrow references to the elements of a vector, you can iterate over the indices of a vector using a Range (e.g., 0..v.len()). This allows you to access elements directly via indexing, which can be useful when you need to work with indices explicitly.

// The issue arises because:
// v.iter() creates an immutable borrow of v for iteration.
// v.push(*n_ref) attempts to mutate v while it’s still immutably borrowed, which violates Rust's borrowing rules.
fn iterator_3() {
    fn dup_in_place(v: &mut Vec<i32>) {
        // Solution 1: Use a Temporary Copy for Iteration
        let original = v.clone(); // Clone the vector to iterate over a temporary copy.
        for n in original.iter() {
            v.push(*n); // Mutably modify the original vector.
        }

        // Solution 2: Use an Index-Based Loop
        let len = v.len(); // Get the original length of the vector.
        for i in 0..len {
            v.push(v[i]); // Access elements by index and push them to the vector.
        }
    }
}
// Context: In Rust, iterating over a collection with .iter() borrows the collection immutably (&), which removes the ability to modify it within the same scope. This ensures that Rust prevents mutable and immutable borrows of the same data simultaneously, maintaining memory safety.

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
