// Solution: use borrowing to avoid moving the value of x
#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String),
}

fn main() {
    let x = Either::Right(String::from("Hello world"));

    // Borrow x to avoid moving its content
    let value = match &x {
        Either::Left(n) => *n,       // Use dereference to get the value for Left
        Either::Right(s) => s.len(), // Borrow the String to calculate its length
    };
    println!("{x:?} {value}");
}
// Context: The code does not compile because the match expression moves the value of the String stored in the Either::Right variant. Once the value is moved, the x variable is no longer valid for use, causing the compilation error.
