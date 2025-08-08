// Week 4: Iterators and Closures
// Learn about functional programming features in Rust

use std::collections::HashMap;

// TODO 1: Basic closures
fn basic_closures() {
    println!("--- Basic Closures ---");
    
    // Simple closure
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));
    
    // Closure with explicit types
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("3 * 4 = {}", multiply(3, 4));
    
    // Closure that captures environment
    let factor = 10;
    let scale = |x| x * factor;
    println!("5 * 10 = {}", scale(5));
    
    // Closure with multiple statements
    let complex_calc = |x: i32| {
        let temp = x * 2;
        let result = temp + 1;
        result
    };
    println!("Complex calc(5) = {}", complex_calc(5));
}

// TODO 2: Closure capture modes
fn closure_capture_modes() {
    println!("\n--- Closure Capture Modes ---");
    
    // By reference (default)
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    
    // Move closure
    let list2 = vec![4, 5, 6];
    println!("Before move closure: {:?}", list2);
    
    let take_ownership = move || {
        println!("From move closure: {:?}", list2);
    };
    take_ownership();
    // list2 can't be used here anymore
    
    // Mutable reference
    let mut list3 = vec![7, 8, 9];
    let mut borrows_mutably = || {
        list3.push(10);
    };
    borrows_mutably();
    println!("After mutable borrow: {:?}", list3);
}

// TODO 3: Functions accepting closures
fn apply_operation<F>(x: i32, y: i32, op: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    op(x, y)
}

fn call_with_one<F>(f: F) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(1)
}

fn call_multiple_times<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
    f();
}

// TODO 4: Returning closures
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn create_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

// TODO 5: Basic iterator usage
fn basic_iterators() {
    println!("\n--- Basic Iterators ---");
    
    let vec = vec![1, 2, 3, 4, 5];
    
    // Iterator methods
    println!("Original vector: {:?}", vec);
    
    // iter() - creates iterator over references
    let sum: i32 = vec.iter().sum();
    println!("Sum using iter(): {}", sum);
    
    // into_iter() - creates iterator that takes ownership
    let vec2 = vec![1, 2, 3, 4, 5];
    let collected: Vec<i32> = vec2.into_iter().collect();
    println!("Collected from into_iter(): {:?}", collected);
    
    // iter_mut() - creates iterator over mutable references
    let mut vec3 = vec![1, 2, 3, 4, 5];
    for item in vec3.iter_mut() {
        *item *= 2;
    }
    println!("After iter_mut() modification: {:?}", vec3);
}

// TODO 6: Iterator adaptors
fn iterator_adaptors() {
    println!("\n--- Iterator Adaptors ---");
    
    let vec = vec![1, 2, 3, 4, 5];
    
    // map - transform each element
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // filter - keep elements matching predicate
    let evens: Vec<&i32> = vec.iter().filter(|&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
    
    // enumerate - add indices
    let with_index: Vec<(usize, &i32)> = vec.iter().enumerate().collect();
    println!("With index: {:?}", with_index);
    
    // zip - combine with another iterator
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let pairs: Vec<(&i32, &char)> = vec.iter().zip(letters.iter()).collect();
    println!("Zipped: {:?}", pairs);
    
    // chain - concatenate iterators
    let vec2 = vec![6, 7, 8];
    let chained: Vec<&i32> = vec.iter().chain(vec2.iter()).collect();
    println!("Chained: {:?}", chained);
    
    // take - take first n elements
    let first_three: Vec<&i32> = vec.iter().take(3).collect();
    println!("First three: {:?}", first_three);
    
    // skip - skip first n elements
    let skip_two: Vec<&i32> = vec.iter().skip(2).collect();
    println!("Skip two: {:?}", skip_two);
}

// TODO 7: Consuming adaptors
fn consuming_adaptors() {
    println!("\n--- Consuming Adaptors ---");
    
    let vec = vec![1, 2, 3, 4, 5];
    
    // collect - consume into collection
    let doubled_vec: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("Collected doubled: {:?}", doubled_vec);
    
    // reduce/fold - accumulate values
    let sum = vec.iter().fold(0, |acc, x| acc + x);
    println!("Fold sum: {}", sum);
    
    let product = vec.iter().fold(1, |acc, x| acc * x);
    println!("Fold product: {}", product);
    
    // reduce with no initial value
    let sum2 = vec.iter().cloned().reduce(|acc, x| acc + x);
    println!("Reduce sum: {:?}", sum2);
    
    // find - find first matching element
    let found = vec.iter().find(|&&x| x > 3);
    println!("First > 3: {:?}", found);
    
    // any/all - boolean tests
    let has_even = vec.iter().any(|&x| x % 2 == 0);
    let all_positive = vec.iter().all(|&x| x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);
    
    // count - count elements
    let count = vec.iter().filter(|&&x| x > 2).count();
    println!("Count > 2: {}", count);
    
    // max/min
    let max_val = vec.iter().max();
    let min_val = vec.iter().min();
    println!("Max: {:?}, Min: {:?}", max_val, min_val);
}

// TODO 8: Custom iterators
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
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

// TODO 9: Iterator for custom data structure
struct Matrix {
    data: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        let mut data = Vec::new();
        for i in 0..rows {
            let mut row = Vec::new();
            for j in 0..cols {
                row.push((i * cols + j) as i32);
            }
            data.push(row);
        }
        Matrix { data }
    }
    
    fn iter(&self) -> MatrixIterator {
        MatrixIterator {
            matrix: self,
            row: 0,
            col: 0,
        }
    }
}

struct MatrixIterator<'a> {
    matrix: &'a Matrix,
    row: usize,
    col: usize,
}

impl<'a> Iterator for MatrixIterator<'a> {
    type Item = &'a i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.matrix.data.len() {
            return None;
        }
        
        if self.col >= self.matrix.data[self.row].len() {
            self.row += 1;
            self.col = 0;
            return self.next();
        }
        
        let item = &self.matrix.data[self.row][self.col];
        self.col += 1;
        Some(item)
    }
}

// TODO 10: Functional programming style
fn functional_programming_examples() {
    println!("\n--- Functional Programming Style ---");
    
    // Process a list of numbers
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // Keep even numbers
        .map(|&x| x * x)           // Square them
        .filter(|&x| x > 10)       // Keep those > 10
        .collect();
    
    println!("Even squares > 10: {:?}", result);
    
    // Work with strings
    let words = vec!["hello", "world", "rust", "is", "awesome"];
    
    let long_words: Vec<String> = words
        .iter()
        .filter(|word| word.len() > 4)
        .map(|word| word.to_uppercase())
        .collect();
    
    println!("Long words (uppercase): {:?}", long_words);
    
    // Group by length
    let mut word_groups: HashMap<usize, Vec<&str>> = HashMap::new();
    
    for word in &words {
        word_groups.entry(word.len()).or_insert(Vec::new()).push(word);
    }
    
    println!("Words grouped by length: {:?}", word_groups);
    
    // Using iterators for grouping
    let grouped: HashMap<usize, Vec<&str>> = words
        .iter()
        .fold(HashMap::new(), |mut acc, &word| {
            acc.entry(word.len()).or_insert(Vec::new()).push(word);
            acc
        });
    
    println!("Functional grouping: {:?}", grouped);
}

// TODO 11: Performance comparison
fn performance_comparison() {
    println!("\n--- Performance Comparison ---");
    
    let large_vec: Vec<i32> = (0..1000000).collect();
    
    // Imperative style
    let start = std::time::Instant::now();
    let mut sum = 0;
    for &item in &large_vec {
        if item % 2 == 0 {
            sum += item * item;
        }
    }
    let imperative_time = start.elapsed();
    println!("Imperative sum: {} (time: {:?})", sum, imperative_time);
    
    // Functional style
    let start = std::time::Instant::now();
    let functional_sum: i32 = large_vec
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    let functional_time = start.elapsed();
    println!("Functional sum: {} (time: {:?})", functional_sum, functional_time);
    
    println!("Results match: {}", sum == functional_sum);
}

// TODO 12: Complex closure examples
fn complex_closure_examples() {
    println!("\n--- Complex Closure Examples ---");
    
    // Closure that captures multiple variables
    let base = 10;
    let multiplier = 2;
    let transformer = |x: i32| (x + base) * multiplier;
    
    let numbers = vec![1, 2, 3, 4, 5];
    let transformed: Vec<i32> = numbers.iter().map(|&x| transformer(x)).collect();
    println!("Transformed: {:?}", transformed);
    
    // Closure factory
    let create_validator = |min: i32, max: i32| {
        move |value: i32| value >= min && value <= max
    };
    
    let is_percentage = create_validator(0, 100);
    let is_age = create_validator(0, 150);
    
    println!("50 is valid percentage: {}", is_percentage(50));
    println!("200 is valid age: {}", is_age(200));
    
    // Higher-order function with closure
    let apply_twice = |f: &dyn Fn(i32) -> i32, x: i32| f(f(x));
    let double = |x: i32| x * 2;
    
    println!("Apply double twice to 3: {}", apply_twice(&double, 3));
}

fn main() {
    println!("=== Iterators and Closures in Rust ===\n");
    
    basic_closures();
    closure_capture_modes();
    
    // TODO 13: Functions with closures
    println!("\n--- Functions with Closures ---");
    let add = |x, y| x + y;
    let multiply = |x, y| x * y;
    
    println!("3 + 5 = {}", apply_operation(3, 5, add));
    println!("3 * 5 = {}", apply_operation(3, 5, multiply));
    
    let square = |x| x * x;
    println!("Square of 1: {}", call_with_one(square));
    
    let mut counter = 0;
    call_multiple_times(|| {
        counter += 1;
        println!("Called {} times", counter);
    });
    
    // TODO 14: Returning closures
    println!("\n--- Returning Closures ---");
    let add_five = create_adder(5);
    println!("5 + 3 = {}", add_five(3));
    
    let mut counter_closure = create_counter();
    println!("Count: {}", counter_closure());
    println!("Count: {}", counter_closure());
    println!("Count: {}", counter_closure());
    
    basic_iterators();
    iterator_adaptors();
    consuming_adaptors();
    
    // TODO 15: Custom iterators
    println!("\n--- Custom Iterators ---");
    let counter = Counter::new(5);
    let collected: Vec<usize> = counter.collect();
    println!("Custom counter: {:?}", collected);
    
    let matrix = Matrix::new(3, 3);
    let matrix_values: Vec<&i32> = matrix.iter().collect();
    println!("Matrix values: {:?}", matrix_values);
    
    functional_programming_examples();
    performance_comparison();
    complex_closure_examples();
    
    println!("\n=== All iterator and closure examples completed! ===");
}
