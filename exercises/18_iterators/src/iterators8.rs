fn main() {
    // Why does the following not compile?
    fn dup_in_place(v: &mut Vec<i32>) {
        for n_ref in v.iter() {
            // Immutable borrow here.
            v.push(*n_ref); // Mutable borrow here, causing a conflict.
        }
    }
}
