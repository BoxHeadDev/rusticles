enum Location {
    Point(i32),
    Range(i32, i32),
}

// What will the output be?
fn main() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::Point(_) => -1,
        Location::Range(_, n) => n,
        Location::Range(0, _) => 0,
        _ => -2,
    };
    println!("{n}");
}
