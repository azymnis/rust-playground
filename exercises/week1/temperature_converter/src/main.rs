// Week 1: Temperature Converter Exercise
// This exercise helps you practice functions, basic math operations, and formatting output

use std::io;

fn main() {
    println!("=== Temperature Converter ===");
    println!("Convert between Celsius, Fahrenheit, and Kelvin\n");

    // Part 1: Basic Conversions
    println!("Part 1: Basic Conversions");
    println!("------------------------");
    
    // Test values
    let celsius_temp = 25.0;
    let fahrenheit_temp = 77.0;
    let kelvin_temp = 298.15;
    
    // TODO: Call conversion functions and print results
    // Example output: "25°C = 77°F"
    
    // Convert Celsius to Fahrenheit
    let c_to_f = celsius_to_fahrenheit(celsius_temp);
    println!("{}°C = {:.1}°F", celsius_temp, c_to_f);
    
    // TODO: Convert Celsius to Kelvin
    // let c_to_k = celsius_to_kelvin(celsius_temp);
    // println!("{}°C = {:.2}K", celsius_temp, c_to_k);
    
    // TODO: Convert Fahrenheit to Celsius
    // let f_to_c = fahrenheit_to_celsius(fahrenheit_temp);
    // println!("{}°F = {:.1}°C", fahrenheit_temp, f_to_c);
    
    // TODO: Convert Fahrenheit to Kelvin
    
    // TODO: Convert Kelvin to Celsius
    
    // TODO: Convert Kelvin to Fahrenheit
    
    println!();

    // Part 2: Temperature Scale Information
    println!("Part 2: Temperature Scale Information");
    println!("------------------------------------");
    
    // TODO: Call the display_freezing_points function
    // display_freezing_points();
    
    // TODO: Call the display_boiling_points function
    // display_boiling_points();
    
    println!();

    // Part 3: Interactive Converter
    println!("Part 3: Interactive Converter");
    println!("----------------------------");
    
    // TODO: Implement interactive temperature conversion
    // run_interactive_converter();
}

// Conversion Functions
// TODO: Implement all conversion functions

// Celsius to Fahrenheit: F = C × 9/5 + 32
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// TODO: Celsius to Kelvin: K = C + 273.15
fn celsius_to_kelvin(celsius: f64) -> f64 {
    // TODO: Implement this function
    0.0
}

// TODO: Fahrenheit to Celsius: C = (F - 32) × 5/9
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // TODO: Implement this function
    0.0
}

// TODO: Fahrenheit to Kelvin: K = (F - 32) × 5/9 + 273.15
fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    // TODO: Implement this function
    // Hint: You can reuse other conversion functions
    0.0
}

// TODO: Kelvin to Celsius: C = K - 273.15
fn kelvin_to_celsius(kelvin: f64) -> f64 {
    // TODO: Implement this function
    0.0
}

// TODO: Kelvin to Fahrenheit: F = (K - 273.15) × 9/5 + 32
fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    // TODO: Implement this function
    // Hint: You can reuse other conversion functions
    0.0
}

// Display Functions
// TODO: Implement these functions to display common reference points

fn display_freezing_points() {
    println!("Freezing point of water:");
    // TODO: Display freezing point in all three scales
    // Water freezes at 0°C, 32°F, 273.15K
    println!("  Celsius: 0°C");
    // TODO: Add Fahrenheit
    // TODO: Add Kelvin
}

fn display_boiling_points() {
    println!("Boiling point of water (at sea level):");
    // TODO: Display boiling point in all three scales
    // Water boils at 100°C, 212°F, 373.15K
    // TODO: Add all three scales
}

// Interactive Converter Function
fn run_interactive_converter() {
    println!("Enter a temperature value followed by the scale (C, F, or K):");
    println!("Example: 25 C or 77 F or 298.15 K");
    println!("Type 'quit' to exit\n");
    
    loop {
        println!("Enter temperature (or 'quit'):");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        
        if input.to_lowercase() == "quit" {
            println!("Goodbye!");
            break;
        }
        
        // TODO: Parse the input to extract temperature value and scale
        // Hint: Use split_whitespace() to separate value and scale
        
        // TODO: Convert to other scales based on input scale
        // Use match expression to handle C, F, or K
        
        // TODO: Display the conversions
        
        println!(); // Empty line for readability
    }
}

// Bonus Exercise: Temperature Comparison
// TODO: Implement a function that takes two temperatures (with their scales)
// and determines which one is hotter
fn compare_temperatures(temp1: f64, scale1: char, temp2: f64, scale2: char) -> String {
    // TODO: Convert both temperatures to the same scale (e.g., Celsius)
    // TODO: Compare and return which is hotter
    // Example: "25°C is cooler than 80°F"
    String::from("Not implemented")
}

// Bonus Exercise: Temperature Range Validator
// TODO: Implement a function that checks if a temperature is physically possible
// Absolute zero: -273.15°C, -459.67°F, 0K
fn is_valid_temperature(temp: f64, scale: char) -> bool {
    // TODO: Check if temperature is above absolute zero for the given scale
    match scale {
        'C' | 'c' => temp >= -273.15,
        'F' | 'f' => temp >= -459.67,
        'K' | 'k' => temp >= 0.0,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(25.0), 77.0);
    }
    
    // TODO: Add more tests for other conversion functions
    
    #[test]
    fn test_is_valid_temperature() {
        assert!(is_valid_temperature(25.0, 'C'));
        assert!(is_valid_temperature(0.0, 'K'));
        assert!(!is_valid_temperature(-300.0, 'C'));
        assert!(!is_valid_temperature(-500.0, 'F'));
        assert!(!is_valid_temperature(-1.0, 'K'));
    }
}
