// Week 2, Exercise 2: Borrowing and References
//
// Learn the difference between immutable and mutable borrows.
// Fix the compilation errors to understand borrowing rules.

fn main() {
    // Part 1: Immutable borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Borrow s1 immutably
    println!("The length of '{}' is {}.", s1, len);  // s1 is still valid!
    
    // Part 2: Multiple immutable borrows are allowed
    let s2 = String::from("world");
    let r1 = &s2;
    let r2 = &s2;
    println!("r1: {}, r2: {}", r1, r2);  // This works!
    
    // Part 3: Mutable borrowing
    let mut s3 = String::from("hello");
    change(&mut s3);
    println!("Changed string: {}", s3);
    
    // Part 4: Mutable borrow rules - only one at a time!
    let mut s4 = String::from("mutable");
    let r3 = &mut s4;
    // TODO: Fix this - can't have two mutable borrows
    // let r4 = &mut s4;
    // println!("r3: {}, r4: {}", r3, r4);
    
    // Part 5: Can't mix mutable and immutable borrows
    let mut s5 = String::from("mixed");
    let r5 = &s5;      // Immutable borrow
    let r6 = &s5;      // Another immutable borrow - OK
    // TODO: Fix this - can't have mutable borrow while immutable borrows exist
    // let r7 = &mut s5;
    println!("r5: {}, r6: {}", r5, r6);
    // r5 and r6 are no longer used after this point
    let r7 = &mut s5;  // Now this is OK!
    println!("r7: {}", r7);
    
    // Part 6: References must be valid
    // TODO: Fix this function to avoid dangling reference
    // let reference_to_nothing = dangle();
    
    // Challenge 1: Write a function that borrows a string slice and returns the first word
    let sentence = String::from("Hello world from Rust");
    // let word = first_word(&sentence);
    // println!("First word: {}", word);
    
    // Challenge 2: Fix this code that tries to modify through an immutable borrow
    let mut data = vec![1, 2, 3, 4, 5];
    let first = &data[0];
    // TODO: This won't compile - why?
    // data.push(6);
    // println!("First element: {}", first);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it doesn't own the value, so nothing happens

fn change(s: &mut String) {
    s.push_str(", world");
}

// TODO: Fix this function - it tries to return a reference to local data
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope and is dropped, so reference would be invalid

// TODO: Implement this function
// fn first_word(s: &String) -> &str {
//     // Hint: use string slicing and look for the first space
// }