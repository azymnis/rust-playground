// Week 4: Modules and Crates
// Learn about Rust's module system and code organization

// TODO 1: Basic module definition
mod basic_module {
    pub fn public_function() {
        println!("Public function in basic_module");
        private_function();
    }
    
    fn private_function() {
        println!("Private function in basic_module");
    }
    
    pub mod nested {
        pub fn nested_function() {
            println!("Function in nested module");
            super::private_function(); // Access parent's private function
        }
    }
}

// TODO 2: Module with structs and enums
mod data_structures {
    pub struct PublicStruct {
        pub public_field: String,
        private_field: i32,
    }
    
    impl PublicStruct {
        pub fn new(public_field: String) -> PublicStruct {
            PublicStruct {
                public_field,
                private_field: 42,
            }
        }
        
        pub fn get_private_field(&self) -> i32 {
            self.private_field
        }
    }
    
    pub enum PublicEnum {
        Variant1,
        Variant2(String),
        Variant3 { field: i32 },
    }
}

// TODO 3: Re-exporting
mod math_operations {
    pub mod arithmetic {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
    }
    
    pub mod advanced {
        pub fn power(base: i32, exp: u32) -> i32 {
            base.pow(exp)
        }
        
        pub fn factorial(n: u32) -> u32 {
            if n <= 1 { 1 } else { n * factorial(n - 1) }
        }
    }
    
    // Re-export commonly used functions
    pub use arithmetic::{add, subtract};
    pub use advanced::power;
}

// TODO 4: Using external paths
use std::collections::HashMap;
use std::io::{self, Write}; // Nested import

// TODO 5: Module in separate files would be declared like this:
// mod external_module; // This would look for external_module.rs or external_module/mod.rs

// TODO 6: Constants and statics in modules
mod constants {
    pub const PI: f64 = 3.14159;
    pub const E: f64 = 2.71828;
    
    pub static GLOBAL_COUNTER: std::sync::atomic::AtomicUsize = 
        std::sync::atomic::AtomicUsize::new(0);
    
    pub fn increment_counter() -> usize {
        GLOBAL_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

// TODO 7: Traits in modules
mod traits {
    pub trait Drawable {
        fn draw(&self);
        
        fn area(&self) -> f64 {
            0.0 // Default implementation
        }
    }
    
    pub struct Circle {
        pub radius: f64,
    }
    
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) {
            println!("Drawing a circle with radius {}", self.radius);
        }
        
        fn area(&self) -> f64 {
            constants::PI * self.radius * self.radius
        }
    }
    
    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("Drawing a rectangle {}x{}", self.width, self.height);
        }
        
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }
}

// TODO 8: Module aliases
mod utils {
    pub mod string_utils {
        pub fn capitalize(s: &str) -> String {
            if s.is_empty() {
                return String::new();
            }
            let mut chars = s.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        }
        
        pub fn word_count(s: &str) -> usize {
            s.split_whitespace().count()
        }
    }
    
    pub mod file_utils {
        use std::fs;
        use std::io;
        
        pub fn create_temp_file(content: &str) -> io::Result<()> {
            fs::write("temp.txt", content)
        }
        
        pub fn read_temp_file() -> io::Result<String> {
            fs::read_to_string("temp.txt")
        }
        
        pub fn delete_temp_file() -> io::Result<()> {
            fs::remove_file("temp.txt")
        }
    }
}

// Create alias for long module path
use utils::string_utils as str_utils;

// TODO 9: Conditional compilation
#[cfg(feature = "advanced")]
mod advanced_features {
    pub fn advanced_function() {
        println!("This function is only available with the 'advanced' feature");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_math_operations() {
        assert_eq!(math_operations::add(2, 3), 5);
        assert_eq!(math_operations::subtract(5, 3), 2);
        assert_eq!(math_operations::power(2, 3), 8);
    }
    
    #[test]
    fn test_string_utils() {
        assert_eq!(str_utils::capitalize("hello"), "Hello");
        assert_eq!(str_utils::word_count("hello world rust"), 3);
    }
}

// TODO 10: Glob imports (use with caution)
mod example_glob {
    pub fn function_a() {}
    pub fn function_b() {}
    pub fn function_c() {}
}

// Import everything from example_glob (not recommended for production)
use example_glob::*;

fn main() {
    println!("=== Modules and Crates in Rust ===\n");
    
    // TODO 11: Using basic modules
    println!("--- Basic Module Usage ---");
    basic_module::public_function();
    basic_module::nested::nested_function();
    
    // TODO 12: Using data structures from modules
    println!("\n--- Data Structures from Modules ---");
    let mut public_struct = data_structures::PublicStruct::new("Hello".to_string());
    println!("Public field: {}", public_struct.public_field);
    println!("Private field (via getter): {}", public_struct.get_private_field());
    
    let enum_variant = data_structures::PublicEnum::Variant2("World".to_string());
    match enum_variant {
        data_structures::PublicEnum::Variant1 => println!("Variant1"),
        data_structures::PublicEnum::Variant2(s) => println!("Variant2: {}", s),
        data_structures::PublicEnum::Variant3 { field } => println!("Variant3: {}", field),
    }
    
    // TODO 13: Using re-exported functions
    println!("\n--- Re-exported Functions ---");
    println!("5 + 3 = {}", math_operations::add(5, 3));
    println!("2^4 = {}", math_operations::power(2, 4));
    
    // Can still access through full path
    println!("8 - 3 = {}", math_operations::arithmetic::subtract(8, 3));
    println!("5! = {}", math_operations::advanced::factorial(5));
    
    // TODO 14: Using constants from modules
    println!("\n--- Constants from Modules ---");
    println!("PI = {}", constants::PI);
    println!("E = {}", constants::E);
    println!("Counter: {}", constants::increment_counter());
    println!("Counter: {}", constants::increment_counter());
    
    // TODO 15: Using traits from modules
    println!("\n--- Traits from Modules ---");
    use traits::{Drawable, Circle, Rectangle};
    
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };
    
    circle.draw();
    println!("Circle area: {:.2}", circle.area());
    
    rectangle.draw();
    println!("Rectangle area: {:.2}", rectangle.area());
    
    // TODO 16: Using module aliases
    println!("\n--- Module Aliases ---");
    let text = "hello world";
    println!("Original: {}", text);
    println!("Capitalized: {}", str_utils::capitalize(text));
    println!("Word count: {}", str_utils::word_count(text));
    
    // TODO 17: File operations
    println!("\n--- File Operations ---");
    let content = "This is temporary content";
    
    match utils::file_utils::create_temp_file(content) {
        Ok(_) => println!("Temp file created successfully"),
        Err(e) => println!("Failed to create temp file: {}", e),
    }
    
    match utils::file_utils::read_temp_file() {
        Ok(content) => println!("Read from temp file: {}", content),
        Err(e) => println!("Failed to read temp file: {}", e),
    }
    
    match utils::file_utils::delete_temp_file() {
        Ok(_) => println!("Temp file deleted successfully"),
        Err(e) => println!("Failed to delete temp file: {}", e),
    }
    
    // TODO 18: Using HashMap from std
    println!("\n--- External Crate Usage (std) ---");
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 92);
    
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    
    // TODO 19: Glob imports usage
    println!("\n--- Glob Imports (Use Carefully) ---");
    function_a();
    function_b();
    function_c();
    
    // TODO 20: Writing to stdout using io
    print!("\n--- Custom Output --- ");
    io::stdout().flush().unwrap();
    println!("Flushed!");
    
    #[cfg(feature = "advanced")]
    {
        println!("\n--- Advanced Features ---");
        advanced_features::advanced_function();
    }
    
    #[cfg(not(feature = "advanced"))]
    println!("\n--- Advanced features not enabled ---");
    
    println!("\n=== Module system examples completed! ===");
    println!("\nTips for organizing Rust code:");
    println!("1. Use modules to organize related functionality");
    println!("2. Make items public only when necessary");
    println!("3. Use re-exports to create convenient APIs");
    println!("4. Consider using separate files for large modules");
    println!("5. Use 'use' statements to bring items into scope");
    println!("6. Avoid glob imports in production code");
}
