// Week 2, Exercise 5: Introduction to Lifetimes
//
// Lifetimes are Rust's way of ensuring references are valid.
// The compiler often infers them, but sometimes we need to be explicit.
//
// Run with: cargo run
// Check compilation: cargo check

fn main() {
    // Part 1: Lifetimes in practice (compiler infers them)
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
    
    // Part 2: Lifetime scope matters
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Longest: '{}'", result);
    }
    // string2 is out of scope here, but we're not using result anymore
    
    // Part 3: This won't work - result would outlive string2
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result); // Error!
    
    // Part 4: Struct with lifetime
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important part: {}", i.part);
    
    // Part 5: Static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("Static: {}", s);
    
    // Challenge 1: Fix this function that needs lifetime annotations
    // let result = first_word_of_longest(&string1, &string2);
    // println!("First word of longest: '{}'", result);
    
    // Challenge 2: Why doesn't this compile?
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);
    
    // Challenge 3: Implement a function that returns a string slice
    // that's guaranteed to live as long as both inputs
    let s1 = String::from("hello");
    let s2 = String::from("world");
    // let result = combine_refs(&s1, &s2);
    // println!("Combined: {}", result);
}

// Lifetime annotations tell the compiler how references relate to each other
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct holding a reference needs a lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// TODO: Implement this function with proper lifetime annotations
// fn first_word_of_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let longest = if x.len() > y.len() { x } else { y };
//     longest.split_whitespace().next().unwrap_or("")
// }

// TODO: This function is tricky - can you make it work?
// Hint: You might need to return a String instead of &str
// fn combine_refs<'a>(x: &'a str, y: &'a str) -> ??? {
//     format!("{} {}", x, y)
// }

// Lifetime elision rules:
// 1. Each parameter gets its own lifetime
// 2. If one input lifetime, output gets that lifetime  
// 3. If &self or &mut self, output gets self's lifetime

// This works without explicit lifetimes due to rule #1 and #2
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}