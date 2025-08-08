// Week 1, Exercise 3: Functions
//
// Learn about:
// - Function parameters
// - Return values
// - Expressions vs statements
// - Early returns
// - Module system and imports
//
// Run with: cargo run
// Check compilation: cargo check

// Declare the module (tells Rust to look for math_functions.rs)
mod math_functions;

// Import specific functions from the module
use math_functions::{greeting, greet_person, add, multiply, check_even, calculate_area};

fn main() {
    // Task 1: Call a simple function
    greeting();
    
    // Task 2: Function with parameters
    greet_person("Alice");
    
    // Task 3: Function with return value
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // Task 4: Complete these function calls
    let product = multiply(4, 7);
    println!("4 * 7 = {}", product);
    
    let is_even = check_even(42);
    println!("Is 42 even? {}", is_even);

    let area = calculate_area(10, 5);
    println!("Area: {}", area);
    
    // Alternative way: use the module prefix directly
    println!("\nUsing module prefix:");
    math_functions::greeting();
    let sum2 = math_functions::add(10, 20);
    println!("10 + 20 = {}", sum2);
}