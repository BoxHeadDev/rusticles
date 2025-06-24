# Fahrenheit to Celsius Converter Challenge

## Key Features of the Solution

- Using functions (fahrenheit_to_celsius) to improve readability and maintainability.
- Looping (loop) to allow multiple conversions until the user decides to exit.
- Pattern matching (match) for error handling, preventing crashes on invalid input.
- Trimmed input (trim()) to remove unwanted whitespace.
- Graceful exit (exit keyword) allowing the user to quit.


use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("🌡️ Fahrenheit to Celsius Converter");

    loop {
        println!("Enter temperature in Fahrenheit (or type 'exit' to quit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        // Allow user to exit the program
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye! 👋");
            break;
        }

        // Attempt to parse input to f64
        match input.parse::<f64>() {
            Ok(fahrenheit) => {
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{fahrenheit}°F is equal to {:.2}°C", celsius);
            }
            Err(_) => println!("❌ Invalid input! Please enter a valid number."),
        }
    }
}
