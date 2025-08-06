// Week 2, Exercise 3: String and Array Slices
//
// Slices are references to a contiguous sequence of elements.
// They're a key part of writing idiomatic Rust.
//
// Run with: cargo run
// Check compilation: cargo check

fn main() {
    // Part 1: String slices (&str)
    let s = String::from("hello world");
    
    // Create slices using range syntax
    let hello = &s[0..5];  // or &s[..5]
    let world = &s[6..11]; // or &s[6..]
    let whole = &s[..];    // Entire string as slice
    
    println!("hello: '{}', world: '{}', whole: '{}'", hello, world, whole);
    
    // Part 2: String literals are slices
    let literal: &str = "I'm a string slice!";
    println!("String literal: {}", literal);
    
    // Part 3: Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // Elements at index 1 and 2
    assert_eq!(slice, &[2, 3]);
    
    // Part 4: Mutable slices
    let mut arr = [10, 20, 30, 40, 50];
    let slice = &mut arr[1..4];
    slice[0] = 100;  // Changes arr[1] to 100
    println!("Modified array: {:?}", arr);
    
    // Part 5: Common slice patterns
    let text = String::from("The quick brown fox");
    
    // TODO: Implement these functions
    // let first = first_word(&text);
    // println!("First word: '{}'", first);
    
    // let second = second_word(&text);
    // println!("Second word: '{}'", second);
    
    // Challenge 1: Why does this cause a problem?
    let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // Error!
    // println!("the first word is: {}", word);
    
    // Challenge 2: Write a function that returns the longest word
    let sentence = "The extraordinarily magnificent Rust language";
    // let longest = find_longest_word(sentence);
    // println!("Longest word: '{}'", longest);
    
    // Challenge 3: Implement a function to reverse words in a sentence
    let input = "hello brave new world";
    // let reversed = reverse_words(input);
    // println!("Reversed: '{}'", reversed);  // Should print: "world new brave hello"
}

// TODO: Implement these functions

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     
//     &s[..]
// }

// fn second_word(s: &str) -> &str {
//     // Hint: Find the first space, then find the second space or end
// }

// fn find_longest_word(s: &str) -> &str {
//     // Hint: Use split_whitespace() and compare lengths
// }

// fn reverse_words(s: &str) -> String {
//     // Hint: split_whitespace(), collect to Vec, reverse, join
// }