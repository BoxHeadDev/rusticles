// 🦀 Rustlings Challenge: Trait Objects and Draw
//
// You’re building a GUI library where every component can be drawn.
// You'll define a `Draw` trait, and then implement it for multiple types.
// Then, you'll create a screen that holds different UI components using trait objects.

pub trait Draw {
    fn draw(&self);
}

// TODO: Implement the `Draw` trait for the Button struct.
pub struct Button {
    pub label: String,
}

// TODO: Implement the `Draw` trait for the TextField struct.
pub struct TextField {
    pub placeholder: String,
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_with_components() {
        let screen = Screen {
            components: vec![
                Box::new(Button {
                    label: "Submit".to_string(),
                }),
                Box::new(TextField {
                    placeholder: "Enter name".to_string(),
                }),
            ],
        };

        screen.run(); // should compile and run
    }
}
