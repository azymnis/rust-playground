// Week 4: Generics
// Learn about generic programming in Rust for code reusability

use std::fmt::Display;
use std::cmp::PartialOrd;

// TODO 1: Generic functions
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn swap<T>(x: &mut T, y: &mut T) {
    std::mem::swap(x, y);
}

// TODO 2: Generic structs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// Methods for specific generic types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// TODO 3: Multiple generic parameters
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn new(x: T, y: U) -> Self {
        MixedPoint { x, y }
    }
    
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// TODO 4: Generic enums
enum Option2<T> {
    Some(T),
    None,
}

enum Result2<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Option2<T> {
    fn unwrap_or(self, default: T) -> T {
        match self {
            Option2::Some(value) => value,
            Option2::None => default,
        }
    }
    
    fn map<U, F>(self, f: F) -> Option2<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option2::Some(value) => Option2::Some(f(value)),
            Option2::None => Option2::None,
        }
    }
}

// TODO 5: Bounded generics
fn print_and_compare<T: Display + PartialOrd>(x: T, y: T) {
    println!("Comparing {} and {}", x, y);
    if x > y {
        println!("{} is greater", x);
    } else if x < y {
        println!("{} is greater", y);
    } else {
        println!("They are equal");
    }
}

// TODO 6: Where clauses for complex bounds
fn compare_display<T, U>(t: &T, u: &U) -> bool
where
    T: Display + PartialEq<U>,
    U: Display,
{
    println!("Comparing '{}' and '{}'", t, u);
    t == u
}

// TODO 7: Generic traits
trait Summary<T> {
    fn summarize(&self) -> T;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary<String> for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary<usize> for NewsArticle {
    fn summarize(&self) -> usize {
        self.content.len()
    }
}

// TODO 8: Generic data structures
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

// TODO 9: Generic implementations with constraints
impl<T: Clone> Stack<T> {
    fn peek_cloned(&self) -> Option<T> {
        self.items.last().cloned()
    }
    
    fn duplicate_top(&mut self) -> Result<(), &'static str> {
        if let Some(top) = self.peek_cloned() {
            self.push(top);
            Ok(())
        } else {
            Err("Stack is empty")
        }
    }
}

// TODO 10: Lifetime parameters with generics
struct Container<'a, T> {
    data: &'a T,
    name: String,
}

impl<'a, T> Container<'a, T> {
    fn new(data: &'a T, name: String) -> Self {
        Container { data, name }
    }
    
    fn get_data(&self) -> &T {
        self.data
    }
}

fn longest_with_display<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// TODO 11: Associated types vs generics
trait Iterator3<T> {
    fn next(&mut self) -> Option<T>;
}

trait Iterator4 {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct NumberIterator {
    current: usize,
    max: usize,
}

// Using generic parameter
impl Iterator3<usize> for NumberIterator {
    fn next(&mut self) -> Option<usize> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// TODO 12: Higher-order generic functions
fn apply_to_all<T, F>(items: Vec<T>, f: F) -> Vec<T>
where
    F: Fn(T) -> T,
{
    items.into_iter().map(f).collect()
}

fn combine_vectors<T: Clone>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    let mut result = v1;
    result.extend(v2);
    result
}

// TODO 13: Generic closures and function pointers
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn apply_operation<F>(x: i32, y: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    operation(x, y)
}

// TODO 14: Generic type aliases
type StringStack = Stack<String>;
type IntPoint = Point<i32>;
type StringResult<T> = std::result::Result<T, String>;

// TODO 15: Phantom types
use std::marker::PhantomData;

struct PhantomContainer<T> {
    data: String,
    _phantom: PhantomData<T>,
}

impl<T> PhantomContainer<T> {
    fn new(data: String) -> Self {
        PhantomContainer {
            data,
            _phantom: PhantomData,
        }
    }
    
    fn get_data(&self) -> &str {
        &self.data
    }
}

// Type-safe units using phantom types
struct Kilometers;
struct Miles;

struct Distance<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Distance<Unit> {
    fn new(value: f64) -> Self {
        Distance {
            value,
            _unit: PhantomData,
        }
    }
    
    fn value(&self) -> f64 {
        self.value
    }
}

impl Distance<Kilometers> {
    fn to_miles(self) -> Distance<Miles> {
        Distance::new(self.value * 0.621371)
    }
}

impl Distance<Miles> {
    fn to_kilometers(self) -> Distance<Kilometers> {
        Distance::new(self.value * 1.60934)
    }
}

fn main() {
    println!("=== Generics in Rust ===\n");
    
    // TODO 16: Generic functions
    println!("--- Generic Functions ---");
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest char is {}", result);
    
    let mut x = 5;
    let mut y = 10;
    println!("Before swap: x={}, y={}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x={}, y={}", x, y);
    
    // TODO 17: Generic structs
    println!("\n--- Generic Structs ---");
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("Integer point: ({}, {})", integer_point.x(), integer_point.y());
    println!("Float point: ({}, {})", float_point.x(), float_point.y());
    println!("Distance from origin: {}", float_point.distance_from_origin());
    
    // TODO 18: Multiple generic parameters
    println!("\n--- Multiple Generic Parameters ---");
    let p1 = MixedPoint::new(5, 10.4);
    let p2 = MixedPoint::new("Hello", 'c');
    let p3 = p1.mixup(p2);
    println!("Mixed point: x={}, y={}", p3.x, p3.y);
    
    // TODO 19: Generic enums
    println!("\n--- Generic Enums ---");
    let some_number = Option2::Some(5);
    let some_string = Option2::Some("Hello");
    let none_int: Option2<i32> = Option2::None;
    
    println!("some_number: {}", some_number.unwrap_or(0));
    println!("none_int: {}", none_int.unwrap_or(42));
    
    let doubled = Option2::Some(5).map(|x| x * 2);
    println!("Doubled: {}", doubled.unwrap_or(0));
    
    // TODO 20: Bounded generics
    println!("\n--- Bounded Generics ---");
    print_and_compare(10, 20);
    print_and_compare("hello", "world");
    
    // TODO 21: Generic data structures
    println!("\n--- Generic Data Structures ---");
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    
    println!("Stack length: {}", int_stack.len());
    println!("Peek: {:?}", int_stack.peek());
    
    int_stack.duplicate_top().unwrap();
    println!("After duplicate: length = {}", int_stack.len());
    
    while let Some(item) = int_stack.pop() {
        println!("Popped: {}", item);
    }
    
    // TODO 22: String stack using type alias
    println!("\n--- Type Aliases ---");
    let mut string_stack: StringStack = Stack::new();
    string_stack.push("Hello".to_string());
    string_stack.push("World".to_string());
    
    println!("String stack peek: {:?}", string_stack.peek());
    
    // TODO 23: Generic traits
    println!("\n--- Generic Traits ---");
    let article = NewsArticle {
        headline: "Breaking News".to_string(),
        location: "New York".to_string(),
        author: "Jane Doe".to_string(),
        content: "This is a very important news article with lots of content.".to_string(),
    };
    
    let summary_string: String = article.summarize();
    let summary_length: usize = article.summarize();
    
    println!("Article summary: {}", summary_string);
    println!("Content length: {}", summary_length);
    
    // TODO 24: Higher-order generic functions
    println!("\n--- Higher-Order Generic Functions ---");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = apply_to_all(numbers, |x| x * 2);
    println!("Doubled numbers: {:?}", doubled);
    
    let add = |x, y| x + y;
    let multiply = |x, y| x * y;
    
    println!("5 + 3 = {}", apply_operation(5, 3, add));
    println!("5 * 3 = {}", apply_operation(5, 3, multiply));
    
    let times_three = create_multiplier(3);
    println!("7 * 3 = {}", times_three(7));
    
    // TODO 25: Phantom types
    println!("\n--- Phantom Types ---");
    let distance_km = Distance::<Kilometers>::new(100.0);
    let distance_miles = distance_km.to_miles();
    
    println!("100 km = {:.2} miles", distance_miles.value());
    
    // TODO 26: Container with lifetime
    println!("\n--- Container with Lifetime ---");
    let data = 42;
    let container = Container::new(&data, "My Container".to_string());
    println!("Container '{}' holds: {}", container.name, container.get_data());
    
    let result = longest_with_display("long string", "short", "Important announcement!");
    println!("Longest string: {}", result);
    
    println!("\n=== All generic examples completed! ===");
}
