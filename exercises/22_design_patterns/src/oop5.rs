// 🦀 Rustlings Challenge: Trait Object Inference
//
// Trait objects (`Box<dyn Trait>`) allow us to store multiple types that implement a trait,
// but type inference can become ambiguous if not guided.
//
// The following code does not compile due to type inference failure. Your job is to fix it
// by either casting one of the items to `Box<dyn Draw>` or by adding a type annotation
// to the `components` variable.
//
// Hint: The error appears because Rust cannot infer the correct vector element type
// when it's initialized before it’s assigned to a struct expecting `Vec<Box<dyn Draw>>`.

trait Draw {
    fn draw(&self);
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

impl Draw for Button {
    fn draw(&self) {
        println!("Button: {}", self.label);
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox with {} options", self.options.len());
    }
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
    // 🔧 TODO: Fix this compilation error by resolving the trait object type inference issue

    let components = vec![
        Box::new(SelectBox {
            width: 100,
            height: 20,
            options: vec![String::from("Yes"), String::from("No")],
        }),
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("Submit"),
        }),
    ];

    let screen = Screen { components };

    screen.run();
}
