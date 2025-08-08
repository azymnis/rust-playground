// Week 4: Traits Basics
// Learn about Rust's powerful trait system for shared behavior

use std::fmt;

// TODO 1: Define a basic trait
trait Speak {
    fn speak(&self) -> String;
    
    // Default implementation
    fn introduce(&self) -> String {
        format!("Hello, I can speak: {}", self.speak())
    }
}

// TODO 2: Implement trait for different types
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

struct Robot {
    id: u32,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says Woof!", self.name)
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{} says Meow!", self.name)
    }
}

impl Speak for Robot {
    fn speak(&self) -> String {
        format!("Robot-{} says BEEP BOOP", self.id)
    }
    
    // Override default implementation
    fn introduce(&self) -> String {
        format!("SYSTEM ONLINE: {}", self.speak())
    }
}

// TODO 3: Trait bounds in functions
fn make_speak(speaker: &dyn Speak) {
    println!("{}", speaker.speak());
    println!("{}", speaker.introduce());
}

fn make_multiple_speak<T: Speak>(speakers: &[T]) {
    for speaker in speakers {
        println!("{}", speaker.speak());
    }
}

// TODO 4: Multiple trait bounds
trait Fly {
    fn fly(&self) -> String;
}

trait Swim {
    fn swim(&self) -> String;
}

struct Duck {
    name: String,
}

impl Speak for Duck {
    fn speak(&self) -> String {
        format!("{} says Quack!", self.name)
    }
}

impl Fly for Duck {
    fn fly(&self) -> String {
        format!("{} is flying gracefully", self.name)
    }
}

impl Swim for Duck {
    fn swim(&self) -> String {
        format!("{} is swimming in the pond", self.name)
    }
}

// Function with multiple trait bounds
fn versatile_animal<T: Speak + Fly + Swim>(animal: &T) {
    println!("{}", animal.speak());
    println!("{}", animal.fly());
    println!("{}", animal.swim());
}

// TODO 5: Trait as return type
fn create_speaker(animal_type: &str) -> Box<dyn Speak> {
    match animal_type {
        "dog" => Box::new(Dog { name: "Buddy".to_string() }),
        "cat" => Box::new(Cat { name: "Whiskers".to_string() }),
        "robot" => Box::new(Robot { id: 42 }),
        _ => Box::new(Dog { name: "Default".to_string() }),
    }
}

// TODO 6: Associated types
trait Iterator2 {
    type Item; // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator2 for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// TODO 7: Trait with associated constants
trait MathConstants {
    const PI: f64;
    const E: f64 = 2.718281828; // Default value
    
    fn circle_area(radius: f64) -> f64 {
        Self::PI * radius * radius
    }
}

struct StandardMath;

impl MathConstants for StandardMath {
    const PI: f64 = 3.141592654;
}

struct ApproximateMath;

impl MathConstants for ApproximateMath {
    const PI: f64 = 3.14;
    const E: f64 = 2.72; // Override default
}

// TODO 8: Marker traits
trait Safe {}
trait Dangerous {}

struct SafeOperation;
struct DangerousOperation;

impl Safe for SafeOperation {}
impl Dangerous for DangerousOperation {}

fn execute_safe<T: Safe>(_op: T) {
    println!("Executing safe operation");
}

fn execute_dangerous<T: Dangerous>(_op: T) {
    println!("⚠️ Executing dangerous operation with caution");
}

// TODO 9: Blanket implementations
trait Display2 {
    fn display(&self) -> String;
}

// Blanket implementation for all types that implement fmt::Display
impl<T: fmt::Display> Display2 for T {
    fn display(&self) -> String {
        format!("Displaying: {}", self)
    }
}

// TODO 10: Supertraits
trait Animal {
    fn name(&self) -> &str;
}

trait Mammal: Animal { // Mammal is a supertrait of Animal
    fn fur_color(&self) -> &str;
}

struct Bear {
    name: String,
    fur_color: String,
}

impl Animal for Bear {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Mammal for Bear {
    fn fur_color(&self) -> &str {
        &self.fur_color
    }
}

fn describe_mammal<T: Mammal>(mammal: &T) {
    println!("{} is a mammal with {} fur", mammal.name(), mammal.fur_color());
}

// TODO 11: Where clauses
fn complex_function<T, U, V>(t: T, u: U, v: V) -> String 
where
    T: fmt::Display + Clone,
    U: fmt::Debug + PartialEq<V>,
    V: fmt::Debug,
{
    if u == v {
        format!("Equal: {} (cloned: {})", t, t.clone())
    } else {
        format!("Not equal: {} vs {:?} vs {:?}", t, u, v)
    }
}

// TODO 12: Conditional implementations
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// Only implement this method if T implements PartialOrd
impl<T: PartialOrd> Pair<T> {
    fn larger(&self) -> &T {
        if self.x > self.y {
            &self.x
        } else {
            &self.y
        }
    }
}

// TODO 13: Implementing standard library traits
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON && (self.y - other.y).abs() < f64::EPSILON
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance_from_origin().partial_cmp(&other.distance_from_origin())
    }
}

fn main() {
    println!("=== Traits Basics in Rust ===\n");
    
    // TODO 14: Basic trait usage
    println!("--- Basic Trait Usage ---");
    let dog = Dog { name: "Rex".to_string() };
    let cat = Cat { name: "Fluffy".to_string() };
    let robot = Robot { id: 101 };
    
    make_speak(&dog);
    make_speak(&cat);
    make_speak(&robot);
    
    // TODO 15: Multiple trait bounds
    println!("\n--- Multiple Trait Bounds ---");
    let duck = Duck { name: "Donald".to_string() };
    versatile_animal(&duck);
    
    // TODO 16: Trait as return type
    println!("\n--- Trait as Return Type ---");
    let speakers = vec!["dog", "cat", "robot", "unknown"];
    for speaker_type in speakers {
        let speaker = create_speaker(speaker_type);
        println!("{}", speaker.speak());
    }
    
    // TODO 17: Associated types
    println!("\n--- Associated Types ---");
    let mut counter = Counter::new(5);
    while let Some(num) = counter.next() {
        println!("Count: {}", num);
    }
    
    // TODO 18: Associated constants
    println!("\n--- Associated Constants ---");
    println!("Standard PI: {}", StandardMath::PI);
    println!("Approximate PI: {}", ApproximateMath::PI);
    println!("Circle area (r=2): {}", StandardMath::circle_area(2.0));
    
    // TODO 19: Marker traits
    println!("\n--- Marker Traits ---");
    let safe_op = SafeOperation;
    let dangerous_op = DangerousOperation;
    execute_safe(safe_op);
    execute_dangerous(dangerous_op);
    
    // TODO 20: Blanket implementations
    println!("\n--- Blanket Implementations ---");
    let number = 42;
    let text = "Hello";
    println!("{}", number.display());
    println!("{}", text.display());
    
    // TODO 21: Supertraits
    println!("\n--- Supertraits ---");
    let bear = Bear {
        name: "Yogi".to_string(),
        fur_color: "brown".to_string(),
    };
    describe_mammal(&bear);
    
    // TODO 22: Where clauses
    println!("\n--- Where Clauses ---");
    let result1 = complex_function("test", 5, 5);
    let result2 = complex_function("hello", 10, 20);
    println!("{}", result1);
    println!("{}", result2);
    
    // TODO 23: Conditional implementations
    println!("\n--- Conditional Implementations ---");
    let pair_ints = Pair::new(5, 10);
    let pair_strings = Pair::new("hello".to_string(), "world".to_string());
    
    println!("Larger int: {}", pair_ints.larger());
    println!("Larger string: {}", pair_strings.larger());
    
    // TODO 24: Standard library traits
    println!("\n--- Standard Library Traits ---");
    let p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(1.0, 1.0);
    let p3 = Point::new(3.0, 4.0);
    
    println!("Point 1: {}", p1);
    println!("Point 2: {}", p2);
    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);
    println!("p1 > p2: {}", p1 > p2);
    
    println!("\n=== All trait examples completed! ===");
}
