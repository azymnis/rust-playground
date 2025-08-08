// Week 3: Enums Basics
// Learn about enums and their powerful variants

// TODO 1: Define a simple enum
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// TODO 2: Define an enum with data
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// TODO 3: Define an enum with different variant types
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Named fields like a struct
    Write(String),              // Single String
    ChangeColor(i32, i32, i32), // Multiple values
}

// TODO 4: Define an enum for a state machine
#[derive(Debug)]
enum State {
    Waiting,
    Processing(String),
    Completed(String),
    Failed(String),
}

// TODO 5: Option<T> enum usage (built-in enum)
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// TODO 6: Result<T, E> enum usage (built-in enum)
fn check_age(age: i32) -> Result<String, String> {
    if age < 0 {
        Err(String::from("Age cannot be negative"))
    } else if age < 18 {
        Err(String::from("Must be 18 or older"))
    } else {
        Ok(String::from("Access granted"))
    }
}

// TODO 7: Implement methods on enums
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // TODO 8: Create enum instances
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);
    
    // TODO 9: Enums with data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);
    
    // TODO 10: Different variant types
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello!")),
        Message::ChangeColor(255, 0, 128),
    ];
    
    for msg in &messages {
        msg.call();
    }
    
    // TODO 11: Pattern matching with enums
    let msg = Message::Move { x: 3, y: 4 };
    match msg {
        Message::Quit => println!("The Quit variant has no data"),
        Message::Move { x, y } => println!("Move in the x: {} and y: {} direction", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
    }
    
    // TODO 12: Working with Option<T>
    let result = divide(10.0, 2.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }
    
    let result = divide(10.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }
    
    // TODO 13: if let with Option
    let some_number = Some(5);
    if let Some(n) = some_number {
        println!("The number is: {}", n);
    }
    
    // TODO 14: Working with Result<T, E>
    match check_age(25) {
        Ok(msg) => println!("Success: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
    
    match check_age(15) {
        Ok(msg) => println!("Success: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
    
    // TODO 15: State machine example
    let mut state = State::Waiting;
    println!("Initial state: {:?}", state);
    
    state = State::Processing(String::from("Task 1"));
    println!("Processing: {:?}", state);
    
    state = State::Completed(String::from("Task 1 done"));
    println!("Final state: {:?}", state);
}
