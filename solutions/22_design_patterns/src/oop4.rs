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

// ✅ Implemented Draw for Button
impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a Button: '{}' ({}x{})",
            self.label, self.width, self.height
        );
    }
}

// ✅ Implemented Draw for SelectBox
impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a SelectBox ({}x{}) with options: {:?}",
            self.width, self.height, self.options
        );
    }
}

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
            component.draw(); // ✅ Dynamic dispatch via trait object
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
        ],
    };

    screen.run();
}
