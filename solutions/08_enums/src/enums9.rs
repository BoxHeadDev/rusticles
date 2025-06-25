// Solution:
enum Message {
    Quit,                       // Unit variant
    Move { x: i32, y: i32 },    // Struct-like variant
    Write(String),              // Tuple-like variant
    ChangeColor(i32, i32, i32), // Tuple-like variant
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Received Quit message.");
            }
            Message::Move { x, y } => {
                println!("Move to coordinates: ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("Message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB({}, {}, {})", r, g, b);
            }
        }
    }
}

fn main() {
    // Create an instance of the `Message` enum with the `Write` variant.
    let m = Message::Write(String::from("hello"));

    // Call the `call` method on the enum instance.
    m.call(); // Output: "Message: hello"
}
// Context: Enums in Rust can have methods, just like structs. These methods can operate on specific variants of the enum.
