# Fahrenheit to Celsius Converter Challenge

Welcome to this Rust programming challenge! In this task, you'll build a simple **temperature conversion program** that takes user input in **Fahrenheit** and converts it to **Celsius**. This project will help you practice **user input handling, mathematical operations, and Rust's type system**.

## Challenge Overview

Your goal is to write a Rust program that:

1. Prompts the user to enter a temperature in **Fahrenheit**.
2. Reads the user input and ensures it is a valid number.
3. Converts the Fahrenheit temperature to Celsius using the formula:

   \[
   C = \frac{(F - 32) \times 5}{9}
   \]

4. Prints the result to the console.

## Getting Started

### 1. Understanding the Formula

The conversion formula from Fahrenheit to Celsius is:

   \[
   C = \frac{(F - 32) \times 5}{9}
   \]

Where:
- C is the temperature in Celsius.
- F is the temperature in Fahrenheit.

### 2. Implement the Logic

- Use io::stdin() to get input from the user.
- Convert the input from a string to a floating-point number (f64).
- Apply the conversion formula.
- Print the result in a user-friendly format.

### 3. Handling Edge Cases

Your program should handle the following cases:

- Invalid Input Handling: If the user enters a non-numeric value, the program should prompt them to enter a valid number instead of crashing.
- Decimals: Ensure the program correctly handles decimal values (e.g., 32.5°F).
- Negative Temperatures: The program should work with negative values (e.g., -40°F).

## Bonus Challenges

If you complete the basic challenge, try these extra tasks:

- Celsius to Fahrenheit: Allow the user to convert in both directions.
- Loop Until Valid Input: Keep asking for input until the user provides a valid number.
- Display to Two Decimal Places: Format the output to always show two decimal places.


fn main() {
    println!("Hello, world!");
}
