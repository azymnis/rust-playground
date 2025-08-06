// Week 1, Exercise 2: Variables and Mutability
//
// Learn about:
// - Variable declaration
// - Mutability
// - Type inference
// - Shadowing
//
// Fix the compilation errors in this program!
//
// Run with: cargo run
// Check compilation: cargo check

fn main() {
    // ERROR 1: This variable is immutable
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;  // Uncomment and fix this
    
    // Task 1: Make a mutable variable
    // let y = 10;
    // y = 20;
    // println!("y is now: {}", y);
    
    // Task 2: Variable shadowing
    let spaces = "   ";
    // Can we shadow with a different type?
    // let spaces = spaces.len();
    // println!("Number of spaces: {}", spaces);
    
    // Task 3: Constants (must have type annotation)
    // const MAX_POINTS = 100_000;
    // println!("Max points: {}", MAX_POINTS);
    
    // Task 4: Different number types
    // let integer = 42;
    // let float = 3.14;
    // let boolean = true;
    // let character = 'R';
    
    // Task 5: Type annotations
    // let guess = "42".parse().expect("Not a number!");
    // What's wrong with the line above?
}