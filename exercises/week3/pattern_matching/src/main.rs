// Week 3: Pattern Matching
// Master Rust's powerful pattern matching capabilities

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
    // ... etc
}

// TODO 1: Basic match expression
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// TODO 2: Match with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// TODO 3: Match guards
fn describe_number(x: i32) -> &'static str {
    match x {
        n if n < 0 => "negative",
        0 => "zero",
        n if n > 0 && n < 10 => "small positive",
        n if n >= 10 && n < 100 => "medium positive",
        _ => "large positive",
    }
}

// TODO 4: Matching ranges
fn describe_age(age: u32) -> &'static str {
    match age {
        0..=12 => "child",
        13..=19 => "teenager",
        20..=64 => "adult",
        _ => "senior",
    }
}

// TODO 5: Destructuring structs
struct Point {
    x: i32,
    y: i32,
}

fn describe_point(point: Point) -> String {
    match point {
        Point { x: 0, y: 0 } => String::from("origin"),
        Point { x: 0, y } => format!("on y-axis at {}", y),
        Point { x, y: 0 } => format!("on x-axis at {}", x),
        Point { x, y } => format!("at ({}, {})", x, y),
    }
}

// TODO 6: Complex enum matching
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

fn main() {
    // TODO 7: Basic matching
    let number = 7;
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("prime"),
        13..=19 => println!("teen"),
        _ => println!("something else"),
    }
    
    // TODO 8: Match with value binding
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Coin value: {} cents", value_in_cents(coin));
    
    // TODO 9: Match with Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    
    // TODO 10: if let syntax
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    
    // TODO 11: while let syntax
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    // TODO 12: Match guards
    let num = 15;
    println!("{} is {}", num, describe_number(num));
    
    // TODO 13: Range matching
    let age = 25;
    println!("Age {} is {}", age, describe_age(age));
    
    // TODO 14: Destructuring
    let origin = Point { x: 0, y: 0 };
    let point = Point { x: 3, y: 7 };
    
    println!("Origin: {}", describe_point(origin));
    println!("Point: {}", describe_point(point));
    
    // TODO 15: Complex patterns
    inspect_event(WebEvent::PageLoad);
    inspect_event(WebEvent::KeyPress('x'));
    inspect_event(WebEvent::Click { x: 20, y: 80 });
    
    // TODO 16: @ bindings
    let msg = Some(5);
    match msg {
        Some(x @ 3..=7) => println!("Found {} in range [3, 7]", x),
        Some(x @ 10..=12) => println!("Found {} in range [10, 12]", x),
        Some(x) => println!("Found some other value: {}", x),
        None => println!("Found nothing"),
    }
    
    // TODO 17: Ignoring values
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
    
    // TODO 18: Ignoring with ..
    let point_3d = (3, 5, 7);
    match point_3d {
        (x, ..) => println!("x is {}", x),
    }
    
    // TODO 19: Match must be exhaustive
    let optional = Some(7);
    match optional {
        Some(i) => println!("This is a really long string and `i` is {}", i),
        None => {}, // Can't omit this - match must cover all possibilities
    }
    
    // TODO 20: Using _ for catch-all
    let some_value = 0u8;
    match some_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // Do nothing for any other value
    }
}
