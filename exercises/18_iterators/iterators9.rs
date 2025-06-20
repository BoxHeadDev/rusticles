// 🦀 Rustlings Challenge: Basic Iterator
//
// In this challenge, you'll get familiar with creating and using iterators.
// You'll:
// - Create an iterator from a vector
// - Use `next()` to pull values manually
// - Use a `for` loop to consume an iterator
//
// HINTS:
// - `vec.iter()` gives you an iterator that yields references to the elements
// - `next()` returns an `Option<&T>`
// - `for` automatically consumes the iterator

fn main() {
    let v1 = vec![10, 20, 30];

    // ✅ Create an iterator
    let mut iter = v1.iter();

    // ✅ Use next() manually
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&30));
    assert_eq!(iter.next(), None); // iterator is now empty

    let v2 = vec![100, 200, 300];
    let v2_iter = v2.iter();

    // TODO: Use a for loop to print each value in v2_iter
    // Expected output:
    // Got: 100
    // Got: 200
    // Got: 300

    // 🔴 Nothing prints yet!
}
