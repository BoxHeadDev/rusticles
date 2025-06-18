mod aggregator {
    // Define the Summary trait.
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // Our own type, defined in this crate.
    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    // Because both the trait and the type are defined in this crate,
    // we are allowed to implement the trait on the type.
    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // 🚫 This is forbidden by the orphan rule:
    // - `std::fmt::Display` is not local to this crate.
    // - `Vec<i32>` is also not local to this crate.
    // - Therefore, this impl would violate the orphan rule.
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

    // Bring the Summary trait into scope to use its methods.
    println!("1 new post: {}", post.summarize());

    // The below would fail if you tried to implement Display for Vec<i32> in this crate.
    let numbers = vec![1, 2, 3];
    println!("{}", numbers); // Would use std's Display, not your illegal impl.
}
