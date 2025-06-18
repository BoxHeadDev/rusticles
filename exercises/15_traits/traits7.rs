// In this exercise, you're working with a custom trait called `Summary` and a custom type `SocialPost`.
// Your task is to fix the code so that the trait method `summarize` can be called on `post`.
// You'll also learn why Rust doesn't allow you to implement certain traits on certain types.

mod aggregator {
    // Define the Summary trait with a method `summarize`.
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // Define a type `SocialPost` that will implement the trait.
    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    // Implement the trait `Summary` for `SocialPost`.
    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // Why doesn't this work with the Display type?
    impl std::fmt::Display for Vec<i32> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Vector: {:?}", self)
        }
    }
}

use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    // Bring the trait into scope so we can use `summarize` like a method
    println!("1 new post: {}", post.summarize());

    // Try uncommenting the following lines and observe the compiler error.
    let numbers = vec![1, 2, 3];
    println!("{}", numbers); // Why doesn't this work with the custom Display impl above?
}
