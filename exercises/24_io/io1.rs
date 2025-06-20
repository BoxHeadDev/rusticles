// 🦀 Rustlings Challenge: Accepting Command Line Arguments
//
// Your task is to accept two command line arguments passed to this binary,
// and print them as shown below.
//
// ✅ Step 1: Collect the arguments into a `Vec<String>`
// ✅ Step 2: Bind the first real argument to a variable `query`
// ✅ Step 3: Bind the second real argument to a variable `file_path`
// ✅ Step 4: Print them using the same formatting shown below
//
// 💡 Ignore args[0] — it's just the binary name
//
// To test your code:
// Run this file with: `cargo run -- search_term input.txt`
//
// It should print:
// Searching for search_term
// In file input.txt

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: Assign args[1] to `query` and args[2] to `file_path`

    // TODO: Print them as shown below
    // println!("Searching for {query}");
    // println!("In file {file_path}");
}
