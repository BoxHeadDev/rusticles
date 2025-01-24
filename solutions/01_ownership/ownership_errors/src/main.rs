// Solution: The code is fixed by returning the `String` itself, transferring ownership.
fn return_a_string() -> String {
    let s = String::from("Hello world"); // Create a new `String`.
    s // Return the `String` itself, transferring ownership to the caller.
}
// Context: The code attempted to return a reference to a local variable, but local variables are deallocated when the function ends. Returning a reference to such data would result in a dangling reference.

// Mutating read only data
// name in helper function is an immutable reference
// The function is fixed by working with immutable references and cloning data when needed.

fn ownership_error1() {
    let name = vec![String::from("Ferris")];
    let first = &name[0]; // Immutable borrow of the first element.
    stringify_name_with_title(&name); // This function no longer mutates `name`.
    println!("{}", first); // Safe to use `first` because no mutation occurred.
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" "); // Use `join` to create a copy of the data in `name`.
    full.push_str(" Esq."); // Append " Esq." to the copied string.
    full // Return the new string.
}
// Context: The original code attempts to modify a vector (name) while an immutable reference (first) exists, which violates Rust’s borrowing rules. Your task is to fix the function so that it compiles and behaves correctly by avoiding simultaneous mutable and immutable borrowing.

// Solution: The code is fixed by borrowing only the length of the largest string, avoiding conflicts.
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len(); // Get the length only.
    for s in src {
        if s.len() > largest_len {
            // Compare the length without needing the full string reference.
            dst.push(s.clone()); // Safely mutate `dst` because there are no conflicting borrows.
        }
    }
}
// Context: The original code creates an immutable reference to the largest string in a mutable vector (dst) while iterating and mutating the same vector. This violates Rust’s borrowing rules, which prevent simultaneous mutable and immutable borrows.

// Solution: The code is fixed by avoiding an ownership transfer from the reference.
fn ownership_error2() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0]; // Immutable reference to the first element of the vector.
                                // Use the reference directly or clone the string if ownership is needed.
    let s: String = s_ref.clone(); // Clone the string to create a new owned `String`.
    println!("{}", s); // Safely use the new owned `String`.
}
// Context: The original code attempts to create a String by dereferencing an immutable reference (s_ref). This would lead to an ownership conflict and potentially a double free because v is still responsible for the ownership of the string.

// Solution: The code is fixed by using the mutable reference properly and avoiding invalid references.
fn ownership_error3() {
    let mut n = 0;
    let a = &mut n; // Create a mutable reference to `n`.
                    // If `b` needs to be used, we can reassign the reference from `a`.
    let b = a; // `a` is now invalid, but `b` owns the mutable reference.
    *b += 1; // Use `b` to modify `n`.
    println!("{}", b); // Safely use `b`.
}
// Context: Rust enforces a strict rule: there can only be one mutable reference to a variable at a time. In the original code, a is a mutable reference to n, and then b is assigned to a. While this does not create a second mutable reference, it transfers ownership of the mutable reference, leaving a invalid.

// Solution: The code is fixed by ensuring no immutable reference exists while mutating the tuple.
fn ownership_error4() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = name.0.clone(); // Clone the first element to create an owned value.
    name.1.push_str(", Esq."); // Now safe to mutate `name.1` because `first` is independent.
    println!("{first} {}", name.1);
}

fn get_first(name: &(String, String)) -> &String {
    &name.0 // This function remains unchanged as it borrows immutably.
}
// Context: The original code creates an immutable reference (first) to one part of a tuple (name.0) while trying to mutate another part of the same tuple (name.1). This violates Rust's borrowing rules because you cannot have an active immutable reference while also mutating the same data structure.

// Solution: The code is fixed by separating the mutable and immutable borrows.
fn ownership_error5() {
    let mut a = [0, 1, 2, 3];
    {
        let x = &mut a[1]; // Mutable borrow of the second element.
        let y = a[2]; // Copy the value of the third element into `y`.
        *x += y; // Modify the second element using the copied value of the third element.
    }
    // Both borrows have ended here, and further operations on `a` are safe.
    println!("{:?}", a); // Output the modified array.
}
// Context: The original code attempts to create both a mutable reference (x) to one element of an array and an immutable reference (y) to another element. While these references target different indices, Rust's compiler doesn't analyse the indices and assumes potential aliasing, leading to a borrow conflict.

// Solution: The code is fixed by avoiding ownership transfer and instead using cloning if needed.
fn ownership_error6() {
    let s = String::from("Hello world"); // `s` owns the string.
    let s_ref = &s; // Borrow `s` immutably.
    let s2 = s_ref.clone(); // Clone the string instead of moving ownership.
    println!("{s2}"); // Now `s2` has its own copy of the string, and `s` is still valid.
}
// Context: The original code attempts to take ownership of a string via dereferencing an immutable reference, leading to a potential double free issue. This happens because the ownership of the string is moved when you dereference s_ref, but s still exists in scope. Rust prevents this to ensure memory safety.

// Solution: The code is fixed by splitting the mutable borrow into distinct steps.
fn ownership_error7() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1); // Copy the value of `v[0]` into `v[1]`.
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let prev_value = v[i - 1]; // Copy the value of `v[i - 1]` into a temporary variable.
    let n = &mut v[i]; // Create a mutable reference to `v[i]`.
    *n = prev_value; // Safely assign the copied value to `v[i]`.
}
// Context: The original code attempts to create a mutable reference (n) to one element of a vector (v[i]) and then access another element (v[i - 1]) in the same vector. While these indices are different, Rust’s compiler doesn’t verify that, leading to a potential violation of borrowing rules.

// Solution: The code is fixed by modifying a copy of the string instead of attempting to mutate the borrowed value.
fn ownership_error8() {
    let name = String::from("Ferris");
    let name_ref = &name; // Immutable borrow of `name`.
    let phd_name = award_phd(name.clone()); // Clone `name` and pass the owned copy to be modified.
    println!("{}", name_ref); // Safe to use the original `name_ref` as `name` was not modified.
    println!("{}", phd_name); // Print the modified copy of the name with "Ph.D." appended.
}

fn award_phd(name: String) -> String {
    let mut name = name; // Take ownership of the cloned string.
    name.push_str(", Ph.D."); // Safely modify the owned string.
    name // Return the modified string.
}
// Context: The original code attempts to modify a borrowed string (name) by taking ownership of it inside the award_phd function. This violates Rust's borrowing rules and leads to potential memory safety issues, as the ownership of name is transferred while a reference (name_ref) to it still exists.

// Solution: The code is fixed by ensuring all mutable operations are scoped correctly.
fn ownership_error9() {
    let mut point = [0, 1];
    let mut x = point[0]; // Copy the value of `point[0]` into `x`.
    {
        let y = &mut point[1]; // Borrow `point[1]` mutably within its own scope.
        *y += 1; // Safely modify `point[1]`.
    }
    x += 1; // Modify the independent variable `x`.
    println!("{} {}", point[0], point[1]); // Safely access and print the elements of the array.
}
// Context: The original code attempts to create a mutable reference to point[1] (y) while modifying a copy of point[0] (x). Although x does not directly interact with point, Rust enforces strict borrowing rules that prevent this.

fn main() {
    // Execute the function to see if your changes worked!
    let result = return_a_string();
    println!("{}", result);

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
