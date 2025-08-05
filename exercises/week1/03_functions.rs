// Week 1, Exercise 3: Functions
//
// Learn about:
// - Function parameters
// - Return values
// - Expressions vs statements
// - Early returns

fn main() {
    // Task 1: Call a simple function
    greeting();
    
    // Task 2: Function with parameters
    greet_person("Alice");
    
    // Task 3: Function with return value
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // Task 4: Complete these function calls
    // let product = multiply(4, 7);
    // println!("4 * 7 = {}", product);
    
    // let is_even = check_even(42);
    // println!("Is 42 even? {}", is_even);
}

fn greeting() {
    println!("Hello from a function!");
}

fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

fn add(x: i32, y: i32) -> i32 {
    // Note: no semicolon - this is an expression
    x + y
}

// Task 4: Implement these functions
// fn multiply(x: i32, y: i32) -> i32 {
//     // Your code here
// }

// fn check_even(n: i32) -> bool {
//     // Your code here
// }

// Task 5: Fix this function
// fn calculate_area(width: i32, height: i32) {
//     let area = width * height;
//     area  // What's wrong here?
// }