// 🦀 Rustlings Challenge: Struct Lifetime Annotation
//
// Your task is to complete the struct definition and fix the lifetime issues.
// The `Excerpt` struct should hold a string slice, and Rust needs to know how long the
// reference inside the struct should live.
//
// HINT: You'll need to annotate the struct with a lifetime parameter `'a`.
// Also, make sure that the reference used to create the struct lives long enough.

struct Excerpt<'a> {
    part: &'a str, // ✅ Annotated with the lifetime `'a`, tying it to the lifetime of the data
}

fn main() {
    let story = String::from("It was a dark and stormy night. The rain fell in torrents.");
    let first_part = story.split('.').next().unwrap(); // ✅ `first_part` borrows from `story`

    let highlight = Excerpt {
        part: first_part, // ✅ Lifetime of `part` is tied to `story` via `'a`
    };

    println!("Highlight: {}", highlight.part);
}
