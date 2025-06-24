// 🦀 Rustlings Challenge: Iterator Map and Collect
//
// In this exercise, you'll use `.map()` to transform a vector,
// and `.collect()` to gather the transformed results into a new vector.
//
// Iterators are lazy — calling `.map()` alone doesn't do anything until the
// iterator is consumed. `.collect()` is one such consuming method.
//
// HINTS:
// - Use `.iter()` to get an iterator.
// - Use `.map()` with a closure that adds 1 to each value.
// - Use `.collect()` to turn the resulting iterator into a Vec<i32>.

fn main() {
    let numbers = vec![1, 2, 3];

    // 🔴 This doesn't actually do anything yet:
    numbers.iter().map(|x| x + 1);

    // TODO: Use map and collect to create a new vector with incremented values
    let incremented: Vec<i32> = vec![]; // Fix this line

    assert_eq!(incremented, vec![2, 3, 4]);
}
