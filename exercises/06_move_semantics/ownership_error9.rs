// Fix the code to avoid borrowing conflicts when indexing the vector.
// The goal is to safely copy one element of the vector into another without violating borrowing rules.
fn main() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1); // Copy the value of `v[0]` into `v[1]`.
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i]; // Mutable borrow of `v[i]`.
    *n = v[i - 1]; // ERROR: Immutable borrow of `v[i - 1]` conflicts with the mutable borrow of `v[i]`.
}
