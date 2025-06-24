// 🦀 Rustlings Challenge: Iterator Sum
//
// Some iterator methods consume the iterator. These are called consuming adapters.
// One such method is `.sum()`, which takes ownership of the iterator.
//
// Your task is to:
// - Create an iterator from a vector
// - Use `.sum()` to get the total
// - Observe that the iterator is consumed
//
// HINT: Once `.sum()` is called, the iterator can’t be used again.

#[test]
fn iterator_sum() {
    let numbers = vec![4, 5, 6];

    let iter = numbers.iter();

    let total: i32 = iter.sum();

    assert_eq!(total, 15);

    // 🔴 This line would error because `iter` is consumed by `.sum()`
    // assert_eq!(iter.next(), Some(&4));
}
