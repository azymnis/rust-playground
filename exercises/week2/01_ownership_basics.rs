// Week 2, Exercise 1: Ownership Basics
//
// This exercise demonstrates the fundamental ownership rules in Rust.
// Fix the compilation errors to understand how ownership works.

fn main() {
    // Part 1: Move semantics with String (heap-allocated)
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    
    // TODO: Fix this error - s1 is no longer valid
    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    
    // Part 2: Copy semantics with integers (stack-allocated)
    let x = 5;
    let y = x;  // x is copied, not moved
    
    println!("x = {}, y = {}", x, y);  // This works!
    
    // Part 3: Clone to create a deep copy
    let s3 = String::from("world");
    let s4 = s3.clone();  // Explicitly clone the data
    
    println!("s3 = {}, s4 = {}", s3, s4);  // Both are valid!
    
    // Part 4: Ownership and scope
    {
        let s5 = String::from("inner scope");
        // s5 is valid here
    } // s5 goes out of scope and is dropped
    
    // TODO: Fix this - s5 is not valid here
    // println!("s5 = {}", s5);
    
    // Part 5: Understanding what implements Copy
    // Types that implement Copy: integers, floats, bool, char, tuples of Copy types
    let point = (3, 5);  // Tuple of integers
    let another_point = point;  // Copied, not moved
    println!("point = {:?}, another = {:?}", point, another_point);
    
    // Challenge: Create a function that demonstrates ownership transfer
    let my_string = String::from("ownership demo");
    // TODO: Call take_ownership function
    // take_ownership(my_string);
    // TODO: Try to use my_string here - what happens?
    
    // Bonus: Which of these types implement Copy? Test them!
    // - i32, u64, f64
    // - bool, char
    // - &str (string slice)
    // - String
    // - Vec<i32>
    // - (i32, i32)
    // - (i32, String)
}

// TODO: Implement this function
// fn take_ownership(s: String) {
//     println!("I own: {}", s);
// } // s goes out of scope and is dropped