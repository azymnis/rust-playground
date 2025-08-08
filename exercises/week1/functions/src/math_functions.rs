// Module containing math and utility functions

// Public functions - accessible from other modules
pub fn greeting() {
    println!("Hello from a function!");
}

pub fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

pub fn add(x: i32, y: i32) -> i32 {
    // Note: no semicolon - this is an expression
    x + y
}

pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

pub fn check_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn calculate_area(width: i32, height: i32) -> i32 {
    let area = width * height;
    area  // Returns the area value
}

// Unit tests module
// #[cfg(test)] means this module only compiles when running tests
#[cfg(test)]
mod tests {
    // Import all functions from the parent module
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(multiply(0, 100), 0);
        assert_eq!(multiply(-3, 3), -9);
    }

    #[test]
    fn test_check_even() {
        assert_eq!(check_even(2), true);
        assert_eq!(check_even(3), false);
        assert_eq!(check_even(0), true);
        assert_eq!(check_even(-4), true);
        assert_eq!(check_even(-3), false);
    }

    #[test]
    fn test_calculate_area() {
        assert_eq!(calculate_area(5, 10), 50);
        assert_eq!(calculate_area(0, 10), 0);
        assert_eq!(calculate_area(7, 7), 49);
    }

    // Test that demonstrates assert! macro
    #[test]
    fn test_add_is_commutative() {
        let a = 5;
        let b = 10;
        assert!(add(a, b) == add(b, a), "Addition should be commutative");
    }

    // Test that demonstrates multiple assertions
    #[test]
    fn test_even_numbers() {
        let even_numbers = vec![2, 4, 6, 8, 10, 100, 1000];
        for num in even_numbers {
            assert!(check_even(num), "{} should be even", num);
        }
    }

    // Test that demonstrates testing with negative numbers
    #[test]
    fn test_negative_area() {
        // Area with negative dimensions still works mathematically
        assert_eq!(calculate_area(-5, -5), 25);
        assert_eq!(calculate_area(-5, 5), -25);
    }

    // Example of a test that would panic (commented out)
    #[test]
    #[should_panic]
    fn test_that_panics() {
        panic!("This test is expected to panic!");
    }
}