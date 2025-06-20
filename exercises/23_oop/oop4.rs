// 🦀 Rustlings Challenge: Trait Objects for GUI Components
//
// You're building a GUI framework where components can be drawn using a `Draw` trait.
// Implement the `Draw` trait for the provided `Button` and `SelectBox` types.
//
// Then, ensure the `Screen` struct can hold a list of components using trait objects
// and call `.draw()` on each of them when `run()` is called.
//
// 💡 Hint: Trait objects are written using `Box<dyn Trait>`

trait Draw {
    fn draw(&self);
}

// TODO: Implement the Draw trait for these component types.

struct Button {
    width: u32,
    height: u32,
    label: String,
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // 🚫 This line will not compile unless SelectBox and Button implement Draw.
            // ✅ After implementing Draw, you should see draw output when run.
        ],
    };

    screen.run();
}
