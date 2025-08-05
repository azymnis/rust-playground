// Week 2, Exercise 6: Practical Ownership Scenarios
//
// Real-world examples of ownership patterns you'll encounter

use std::collections::HashMap;

fn main() {
    // Scenario 1: Building a string from parts
    let mut output = String::new();
    output.push_str("Hello");
    output.push(' ');
    output.push_str("World");
    
    // TODO: Create a function that joins strings with ownership considerations
    let parts = vec![
        String::from("Rust"),
        String::from("is"),
        String::from("awesome"),
    ];
    // let sentence = join_strings(parts, " ");
    // println!("Joined: {}", sentence);
    // Can you still use 'parts' here?
    
    // Scenario 2: Working with collections
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    
    // TODO: Fix this - how do we update a value?
    let team = String::from("Blue");
    // scores.insert(team, 25);
    // println!("Team {} has score", team); // Error: team was moved!
    
    // Scenario 3: Returning multiple values from a function
    let data = vec![1, 2, 3, 4, 5];
    // let (sum, product, data_back) = calculate_sum_and_product(data);
    // println!("Sum: {}, Product: {}", sum, product);
    // println!("Original data: {:?}", data_back);
    
    // Scenario 4: Builder pattern with ownership
    let config = ConfigBuilder::new()
        .name("MyApp")
        .version("1.0.0")
        .debug(true)
        .build();
    println!("Config: {:?}", config);
    
    // Scenario 5: Processing optional values
    let maybe_name = Some(String::from("Alice"));
    // TODO: How do we use the value inside without moving it?
    // greet_person(maybe_name);
    // greet_person(maybe_name); // Try calling twice!
    
    // Challenge: Implement a cache that borrows keys but owns values
    let mut cache = StringCache::new();
    cache.insert("key1", String::from("value1"));
    // let value = cache.get("key1");
    // println!("Cached value: {:?}", value);
}

// TODO: Implement this function
// Should it take ownership of the vector? Why or why not?
// fn join_strings(strings: Vec<String>, separator: &str) -> String {
//     // Hint: Use iterators and collect()
// }

// TODO: Fix this function to avoid taking ownership of the vector
// fn calculate_sum_and_product(data: Vec<i32>) -> (i32, i32, Vec<i32>) {
//     let sum = data.iter().sum();
//     let product = data.iter().product();
//     (sum, product, data) // Return the vector back
// }

// TODO: How can we make this work with Option<String>?
// fn greet_person(name: Option<String>) {
//     match name {
//         Some(n) => println!("Hello, {}!", n),
//         None => println!("Hello, stranger!"),
//     }
// }

// Builder pattern example
#[derive(Debug)]
struct Config {
    name: String,
    version: String,
    debug: bool,
}

struct ConfigBuilder {
    name: Option<String>,
    version: Option<String>,
    debug: bool,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            name: None,
            version: None,
            debug: false,
        }
    }
    
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    
    fn version(mut self, version: &str) -> Self {
        self.version = Some(version.to_string());
        self
    }
    
    fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
    
    fn build(self) -> Config {
        Config {
            name: self.name.unwrap_or_else(|| "Unnamed".to_string()),
            version: self.version.unwrap_or_else(|| "0.0.0".to_string()),
            debug: self.debug,
        }
    }
}

// TODO: Implement a simple cache
struct StringCache {
    data: HashMap<String, String>,
}

impl StringCache {
    fn new() -> Self {
        StringCache {
            data: HashMap::new(),
        }
    }
    
    // TODO: Should key be &str or String? Why?
    fn insert(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }
    
    // TODO: What should this return? &String? Option<&String>?
    // fn get(&self, key: &str) -> ??? {
    //     
    // }
}