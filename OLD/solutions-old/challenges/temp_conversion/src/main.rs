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
