// 🦀 Rustlings Challenge: Implementing the Draw Trait
//
// Your goal is to define the `Draw` trait and implement it for `Button` and `SelectBox`.
//
// Then, create a `Screen` struct that can hold components implementing the `Draw` trait.
// When calling `run`, it should draw each component by calling `.draw()`.
//
// ✅ Tasks:
// - Define the Draw trait with a method `draw(&self)`.
// - Implement Draw for `Button` and `SelectBox`.
// - Store components in `Screen` as `Box<dyn Draw>` trait objects.
// - Make `Screen::run()` call `.draw()` on each component.

mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in &self.components {
                component.draw();
            }
        }
    }

    // ✅ Define and implement the Button struct
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!(
                "Drawing a button: '{}' ({}x{})",
                self.label, self.width, self.height
            );
        }
    }
}

use gui::{Button, Draw, Screen};

// ✅ Define and implement the SelectBox struct
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// TODO: Implement the Draw trait for SelectBox
// Print something like: "Drawing a select box with options: [...]"

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };

    screen.run();
}
