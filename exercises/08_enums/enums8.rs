// Convert the following structs into variants of a single enum `Message`.
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Define the `Message` enum here, consolidating the above structs.

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
