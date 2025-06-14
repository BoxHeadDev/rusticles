// Solution: The code is fixed by splitting the mutable borrow into distinct steps.
fn main() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1); // Copy the value of `v[0]` into `v[1]`.
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let prev_value = v[i - 1]; // Copy the value of `v[i - 1]` into a temporary variable.
    let n = &mut v[i]; // Create a mutable reference to `v[i]`.
    *n = prev_value; // Safely assign the copied value to `v[i]`.
}
// Context: The original code attempts to create a mutable reference (n) to one element of a vector (v[i]) and then access another element (v[i - 1]) in the same vector. While these indices are different, Rust’s compiler doesn’t verify that, leading to a potential violation of borrowing rules.
