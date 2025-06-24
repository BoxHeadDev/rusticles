// Fix the error by avoiding Moves in Pattern Matching
enum Either {
    Left(usize),
    Right(String),
}

fn main() {
    let x = Either::Right(String::from("Hello world"));
    let value = match x {
        Either::Left(n) => n,
        Either::Right(s) => s.len(),
    };
    println!("{x:?} {value}");
}
