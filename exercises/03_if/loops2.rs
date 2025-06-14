// Fix the code to break out of all loops when `count` is 2
fn main() {
    let mut count = 0;
    loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Breaks only the inner loop
            }
            if count == 2 {
                break; // How can you break out of all loops here?
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // Should print: "End count = 2"
}
