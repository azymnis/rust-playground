// Week 2, Exercise 4: Ownership and Functions
//
// Understanding how ownership works with function parameters and return values
// is crucial for writing Rust programs.

fn main() {
    // Part 1: Passing ownership to functions
    let s = String::from("hello");
    takes_ownership(s);
    // TODO: Fix - s is no longer valid here
    // println!("{}", s);
    
    let x = 5;
    makes_copy(x);
    println!("{}", x);  // x is still valid (Copy trait)
    
    // Part 2: Returning ownership from functions
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    
    println!("s1: {}, s3: {}", s1, s3);
    // TODO: Try to use s2 here
    
    // Part 3: Borrowing in functions is more common
    let s4 = String::from("borrow me");
    let len = calculate_length(&s4);
    println!("Length of '{}' is {}", s4, len);  // s4 is still valid!
    
    // Part 4: Mutable borrowing in functions
    let mut s5 = String::from("change me");
    append_world(&mut s5);
    println!("Changed to: {}", s5);
    
    // Part 5: Multiple return values using tuples
    let s6 = String::from("hello");
    let (s7, len) = calculate_length_owned(s6);
    println!("The length of '{}' is {}", s7, len);
    
    // Challenge 1: Fix this function that tries to return a borrowed value
    let result = create_and_borrow();
    // println!("Result: {}", result);
    
    // Challenge 2: Implement a swap function for two mutable references
    let mut a = String::from("first");
    let mut b = String::from("second");
    // swap(&mut a, &mut b);
    // println!("a: {}, b: {}", a, b);  // Should print: "a: second, b: first"
    
    // Challenge 3: Write a function that takes ownership conditionally
    let valuable = String::from("important data");
    // let result = process_conditionally(valuable, true);
    // match result {
    //     Some(s) => println!("Got back: {}", s),
    //     None => println!("Data was consumed"),
    // }
}

fn takes_ownership(s: String) {
    println!("I own: {}", s);
} // s goes out of scope and is dropped

fn makes_copy(x: i32) {
    println!("I have a copy: {}", x);
} // x goes out of scope, nothing special happens

fn gives_ownership() -> String {
    let s = String::from("yours");
    s  // Return ownership
}

fn takes_and_gives_back(s: String) -> String {
    s  // Return the same string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(" world!");
}

fn calculate_length_owned(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // Return both the string and its length
}

// TODO: Fix this function
fn create_and_borrow() -> String {
    let s = String::from("created inside");
    s  // Return the String, not a reference
}

// TODO: Implement swap function
// fn swap(a: &mut String, b: &mut String) {
//     // Hint: Use a temporary variable or std::mem::swap
// }

// TODO: Implement conditional processing
// fn process_conditionally(data: String, should_return: bool) -> Option<String> {
//     if should_return {
//         // Return the data wrapped in Some
//     } else {
//         println!("Processing and consuming: {}", data);
//         // Return None (data is dropped)
//     }
// }