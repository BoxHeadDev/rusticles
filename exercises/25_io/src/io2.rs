// 🦀 Rustlings Challenge: Read File Contents
//
// Your task is to read the contents of a file from a file path provided as a command-line argument.
// The file to read will be "poem.txt". Print out the file name and its contents.
//
// HINTS:
// - Use `std::env::args` to get the command-line arguments.
// - Use `fs::read_to_string` to read the contents of the file.
// - Handle the `Result` properly using `.expect()` for now.

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: Fix the following line to get the second argument (file path)
    let file_path = ""; // ❌ Not getting the file path from command-line arguments

    println!("In file {file_path}");

    // TODO: Read the file at `file_path` and print its contents
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
