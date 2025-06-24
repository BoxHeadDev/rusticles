fn main() {
    // What are the values of the following iterator?
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();

    let n1: &i32 = iter.next().unwrap(); // What value does this return?
    let n2: &i32 = iter.next().unwrap(); // What value does this return?
    let end: Option<&i32> = iter.next(); // What value does this return?
}
