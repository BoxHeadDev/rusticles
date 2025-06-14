// Solution: Corrected match with reordered patterns
enum Location {
    Point(i32),
    Range(i32, i32),
}

fn main() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::Point(_) => -1,   // Match `Point`
        Location::Range(0, _) => 0, // Match `Range(0, _)` before generic `Range(_, n)`
        Location::Range(_, n) => n, // Match generic `Range(_, n)`
        _ => -2,                    // Default case (not needed here)
    };
    println!("{n}"); // Output will be `0` with the corrected code
    // In the original code, the output is 5 due to pattern ordering
}
// Context: The match expression evaluates patterns in the order they are written. The third pattern (Location::Range(0, _) => 0) is unreachable. The second pattern already matches all Location::Range variants, so it is impossible for the third pattern to ever be executed. Rust's compiler should provide a warning about unreachable code here.
