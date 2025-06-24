// 🦀 Rustlings Challenge: Struct Lifetime Annotation
//
// Your task is to complete the struct definition and fix the lifetime issues.
// The `Excerpt` struct should hold a string slice, and Rust needs to know how long the
// reference inside the struct should live.
//
// HINT: You'll need to annotate the struct with a lifetime parameter `'a`.
// Also, make sure that the reference used to create the struct lives long enough.

struct Excerpt {
    part: &str, // 🛑 ERROR: missing lifetime annotation for this reference
}

fn main() {
    let story = String::from("It was a dark and stormy night. The rain fell in torrents.");
    let first_part = story.split('.').next().unwrap();

    let highlight = Excerpt {
        part: first_part, // ⚠️ This reference must not outlive `story`
    };

    println!("Highlight: {}", highlight.part);
}
