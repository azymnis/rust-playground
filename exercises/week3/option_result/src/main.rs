// Week 3: Option and Result Types
// Master Rust's approach to null values and error handling

use std::fs::File;
use std::io::{self, Read};

// TODO 1: Function returning Option
fn find_first_word(s: &str) -> Option<&str> {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.is_empty() {
        None
    } else {
        Some(words[0])
    }
}

// TODO 2: Function with Option parameter
fn greeting(name: Option<&str>) -> String {
    match name {
        Some(n) => format!("Hello, {}!", n),
        None => String::from("Hello, stranger!"),
    }
}

// TODO 3: Chaining Option methods
fn get_user_age(user_id: u32) -> Option<u32> {
    // Simulated database lookup
    match user_id {
        1 => Some(25),
        2 => Some(30),
        3 => Some(18),
        _ => None,
    }
}

fn can_drink(user_id: u32) -> Option<bool> {
    get_user_age(user_id)
        .map(|age| age >= 21)
}

// TODO 4: Result for error handling
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

// TODO 5: Custom error types
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// TODO 6: Propagating errors with ?
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// TODO 7: Combining Options
struct Person {
    name: String,
    age: Option<u32>,
    email: Option<String>,
}

impl Person {
    fn full_info(&self) -> Option<String> {
        // Only return full info if both age and email are present
        match (self.age, &self.email) {
            (Some(age), Some(email)) => {
                Some(format!("{} is {} years old, email: {}", self.name, age, email))
            }
            _ => None,
        }
    }
}

// TODO 8: Converting between Option and Result
fn parse_age(age_str: &str) -> Option<u32> {
    age_str.parse().ok()  // Convert Result to Option
}

fn get_config_value(key: &str) -> Result<String, String> {
    // Simulated config lookup
    match key {
        "database_url" => Ok(String::from("postgres://localhost")),
        "port" => Ok(String::from("8080")),
        _ => Err(format!("Config key '{}' not found", key)),
    }
}

// TODO 9: Option/Result combinators
fn process_numbers(numbers: Vec<Option<i32>>) -> Option<i32> {
    numbers.into_iter()
        .filter_map(|n| n)  // Remove None values
        .reduce(|acc, x| acc + x)
}

// TODO 10: Early returns with ?
fn complex_calculation(x: f64, y: f64) -> Result<f64, MathError> {
    let divided = divide(x, y).map_err(|_| MathError::DivisionByZero)?;
    let root = sqrt(divided)?;
    Ok(root * 2.0)
}

fn main() {
    // TODO 11: Working with Option
    let text = "Hello world from Rust";
    match find_first_word(text) {
        Some(word) => println!("First word: {}", word),
        None => println!("No words found"),
    }
    
    // TODO 12: Option with default values
    let maybe_name = Some("Alice");
    let no_name: Option<&str> = None;
    
    println!("{}", greeting(maybe_name));
    println!("{}", greeting(no_name));
    
    // TODO 13: unwrap_or and unwrap_or_else
    let default_name = no_name.unwrap_or("Default");
    println!("Name: {}", default_name);
    
    let computed_default = no_name.unwrap_or_else(|| "Computed Default");
    println!("Name: {}", computed_default);
    
    // TODO 14: Option methods
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    
    println!("x.is_some(): {}", x.is_some());
    println!("y.is_none(): {}", y.is_none());
    
    // TODO 15: map and and_then
    let doubled = x.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);
    
    let can_user_1_drink = can_drink(1);
    println!("User 1 can drink: {:?}", can_user_1_drink);
    
    // TODO 16: Working with Result
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // TODO 17: Result methods
    let result = divide(20.0, 4.0)
        .map(|x| x * 2.0)
        .map_err(|e| format!("Math error: {}", e));
    
    println!("Result after mapping: {:?}", result);
    
    // TODO 18: unwrap and expect
    // Be careful with these - they panic on error!
    let safe_result = divide(100.0, 5.0).unwrap();
    println!("Safe division: {}", safe_result);
    
    let expected_result = divide(50.0, 2.0)
        .expect("This should never fail");
    println!("Expected result: {}", expected_result);
    
    // TODO 19: Custom error handling
    match sqrt(-4.0) {
        Ok(result) => println!("Square root: {}", result),
        Err(e) => println!("Math error: {:?}", e),
    }
    
    // TODO 20: Combining Options
    let person1 = Person {
        name: String::from("Bob"),
        age: Some(35),
        email: Some(String::from("bob@example.com")),
    };
    
    let person2 = Person {
        name: String::from("Charlie"),
        age: Some(28),
        email: None,
    };
    
    println!("Person 1 info: {:?}", person1.full_info());
    println!("Person 2 info: {:?}", person2.full_info());
    
    // TODO 21: Processing collections
    let numbers = vec![Some(1), None, Some(3), Some(4), None];
    let sum = process_numbers(numbers);
    println!("Sum of valid numbers: {:?}", sum);
    
    // TODO 22: Pattern matching with Option and Result
    let config = get_config_value("port");
    if let Ok(port) = config {
        println!("Port: {}", port);
    }
    
    let missing = get_config_value("missing_key");
    if let Err(e) = missing {
        println!("Config error: {}", e);
    }
}
