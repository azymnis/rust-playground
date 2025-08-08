// Week 3: Structs Basics
// Learn about different types of structs and their usage

// TODO 1: Define a classic struct for a Person
struct Person {
    name: String,
    age: u32,
    email: String,
}

// TODO 2: Define a tuple struct for RGB color
struct Color(u8, u8, u8);

// TODO 3: Define a unit struct
struct AlwaysEqual;

// TODO 4: Define a struct with different visibility
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
    isbn: String, // private field
}

// TODO 5: Define a struct that contains another struct
struct Address {
    street: String,
    city: String,
    country: String,
}

struct Employee {
    person: Person,
    address: Address,
    employee_id: u32,
    salary: f64,
}

fn main() {
    // TODO 6: Create instances using field init shorthand
    let name = String::from("Alice");
    let age = 30;
    let email = String::from("alice@example.com");
    
    let person1 = Person {
        name,
        age,
        email,
    };
    
    println!("Person 1: {} is {} years old", person1.name, person1.age);
    
    // TODO 7: Create instance with struct update syntax
    let person2 = Person {
        name: String::from("Bob"),
        ..person1  // This moves person1's email
    };
    
    println!("Person 2: {} is {} years old", person2.name, person2.age);
    
    // TODO 8: Work with tuple structs
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    
    println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("White RGB: ({}, {}, {})", white.0, white.1, white.2);
    
    // TODO 9: Destructuring structs
    let Color(r, g, b) = black;
    println!("Destructured black: R={}, G={}, B={}", r, g, b);
    
    // TODO 10: Pattern matching with structs
    let Person { name, age, .. } = person2;
    println!("Destructured person: {} is {} years old", name, age);
    
    // TODO 11: Create a nested struct
    let employee = Employee {
        person: Person {
            name: String::from("Charlie"),
            age: 35,
            email: String::from("charlie@company.com"),
        },
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("Springfield"),
            country: String::from("USA"),
        },
        employee_id: 12345,
        salary: 75000.0,
    };
    
    println!(
        "Employee {} (ID: {}) lives in {}, {}",
        employee.person.name,
        employee.employee_id,
        employee.address.city,
        employee.address.country
    );
    
    // TODO 12: Debug trait for printing
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let origin = Point { x: 0, y: 0 };
    println!("Origin point: {:?}", origin);
    println!("Origin point (pretty): {:#?}", origin);
}
