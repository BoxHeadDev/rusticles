// Solution: Use a label to break out of the outer loop
fn main() {
    let mut count = 0;
    'counting_up: loop {
        // Label the outer loop as `counting_up`
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Break only the inner loop
            }
            if count == 2 {
                break 'counting_up; // Break out of the outer loop using the label
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // Prints: "End count = 2"
}
// Context: In Rust, breaking out of a nested loop requires a loop label. Loop labels allow you to specify which loop you want to break out of when multiple loops are involved.
