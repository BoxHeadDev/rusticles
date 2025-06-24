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

struct ImportantExcerpt {
    part: &str, // 🛑 ERROR: missing lifetime annotation
}

// TODO: Fix this impl block with proper lifetime annotations.
impl ImportantExcerpt {
    // This method just returns an integer, no need to annotate lifetimes here.
    fn level(&self) -> i32 {
        5
    }

    // TODO: This method returns a reference — you'll need to fix its signature
    // so it correctly returns a string slice with the lifetime of `self`.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
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
