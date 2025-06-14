enum Message {
    Quit,                       // Unit variant
    Move { x: i32, y: i32 },    // Struct-like variant
    Write(String),              // Tuple-like variant
    ChangeColor(i32, i32, i32), // Tuple-like variant
}

impl Message {
    // Implement a method for the `Message` enum.
    fn call(&self) {
        // Match on the enum to handle the `Write` variant.
        match self {
            Message::Quit => {
                println!("Received Quit message.");
            }
            Message::Move { x, y } => {
                println!("Move to coordinates: ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("Message: {}", text); // Print the message stored in the `Write` variant.
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB({}, {}, {})", r, g, b);
            }
        }
    }
}

// Refactor this code to use the `Message` enum instead of the standalone `WriteMessage` struct.
fn main() {
    let m = WriteMessage(String::from("hello")); // Replace this with the `Message` enum.
    m.call(); // This should call the `call` method of the `Message` enum.
}
