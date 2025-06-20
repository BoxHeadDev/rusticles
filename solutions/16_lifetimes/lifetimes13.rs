// 🦀 Rustlings Challenge: Method Lifetime Annotation
//
// You are working with a struct that holds a string slice,
// and you're implementing methods that interact with the borrowed data.
//
// Your goal is to correctly annotate the `impl` block and method signatures
// so that they compile and behave as expected.
//
// HINTS:
// - The struct holds a reference, so it needs a lifetime.
// - You must annotate the `impl` block accordingly.
// - You shouldn't need explicit lifetimes in the method parameters (elision!)
// - Use the return value to reinforce lifetime understanding.

struct ImportantExcerpt<'a> {
    part: &'a str, // ✅ Added lifetime parameter to the struct
}

// ✅ Annotated impl with same lifetime parameter as the struct definition
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        5
        // ✅ No lifetime needed here — return type is `i32` (no references)
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // ✅ Lifetime elision rule #3 applies here
        // The return type is tied to `&self`'s lifetime
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: novel.split('.').next().unwrap(), // get "Call me Ishmael"
    };

    println!("Level: {}", excerpt.level());
    println!(
        "Excerpt: {}",
        excerpt.announce_and_return_part("Here's the part:")
    );
}
