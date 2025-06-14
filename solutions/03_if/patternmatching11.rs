// Solution: Refactored function using if let
fn main() {
    let config_max = Some(3u8);

    // Use if let to handle the Some case concisely
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
// Context: if let is a concise and idiomatic way to handle cases where you only care about matching one specific pattern and want to ignore the rest.
