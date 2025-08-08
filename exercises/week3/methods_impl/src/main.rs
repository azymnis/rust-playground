// Week 3: Methods and Implementation Blocks
// Learn how to add methods to structs and enums

// TODO 1: Basic struct with methods
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // TODO 2: Constructor method (associated function)
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    
    // TODO 3: Method that borrows self
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    // TODO 4: Method that borrows self
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    // TODO 5: Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // TODO 6: Mutable method
    fn double_size(&mut self) {
        self.width *= 2.0;
        self.height *= 2.0;
    }
    
    // TODO 7: Associated function (no self)
    fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// TODO 8: Multiple impl blocks (you can have more than one)
impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// TODO 9: Enum with methods
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 50,
        }
    }
    
    fn can_go(&self) -> bool {
        match self {
            TrafficLight::Green => true,
            _ => false,
        }
    }
    
    fn next(&self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
}

// TODO 10: Builder pattern example
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }
    
    fn x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }
    
    fn y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }
    
    fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
    
    fn build(self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// TODO 11: Method chaining
#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }
    
    fn increment(&mut self) -> &mut Self {
        self.value += 1;
        self
    }
    
    fn decrement(&mut self) -> &mut Self {
        self.value -= 1;
        self
    }
    
    fn add(&mut self, n: i32) -> &mut Self {
        self.value += n;
        self
    }
    
    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    // TODO 12: Using constructor
    let rect1 = Rectangle::new(30.0, 50.0);
    println!("Rectangle 1: {:?}", rect1);
    
    // TODO 13: Using methods
    println!("Area: {}", rect1.area());
    println!("Perimeter: {}", rect1.perimeter());
    
    // TODO 14: Using associated function
    let square = Rectangle::square(25.0);
    println!("Square: {:?}", square);
    println!("Is square? {}", square.is_square());
    
    // TODO 15: Method with parameters
    let rect2 = Rectangle::new(20.0, 40.0);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    // TODO 16: Mutable methods
    let mut rect3 = Rectangle::new(10.0, 20.0);
    println!("Before doubling: {:?}", rect3);
    rect3.double_size();
    println!("After doubling: {:?}", rect3);
    
    // TODO 17: Enum methods
    let light = TrafficLight::Red;
    println!("Red light time: {} seconds", light.time());
    println!("Can go? {}", light.can_go());
    
    let next_light = light.next();
    println!("Next light: {:?}", next_light);
    println!("Can go now? {}", next_light.can_go());
    
    // TODO 18: Builder pattern
    let circle = CircleBuilder::new()
        .x(10.0)
        .y(20.0)
        .radius(5.0)
        .build();
    
    println!("Circle: {:?}", circle);
    println!("Circle area: {:.2}", circle.area());
    println!("Circle circumference: {:.2}", circle.circumference());
    
    // TODO 19: Method chaining
    let mut counter = Counter::new();
    counter
        .increment()
        .increment()
        .add(5)
        .decrement();
    
    println!("Counter value: {}", counter.get());
    
    // TODO 20: Self in method signatures
    struct Point {
        x: f64,
        y: f64,
    }
    
    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
        
        // Method that takes ownership
        fn consume(self) -> (f64, f64) {
            (self.x, self.y)
        }
        
        // Method that borrows
        fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
        
        // Method that mutably borrows
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
        }
    }
    
    let mut point = Point::new(3.0, 4.0);
    println!("Distance from origin: {}", point.distance_from_origin());
    
    point.translate(1.0, 1.0);
    println!("After translation: distance = {}", point.distance_from_origin());
    
    let (x, y) = point.consume(); // point is moved here
    println!("Final position: ({}, {})", x, y);
    // point can't be used after consume()
}
