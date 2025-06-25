pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with label: '{}'", self.label);
    }
}

pub struct TextField {
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!(
            "Drawing a text field with placeholder: '{}'",
            self.placeholder
        );
    }
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
