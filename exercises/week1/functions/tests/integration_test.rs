// Integration tests - test the public API from an external perspective
// These tests can only access public functions

// Import the library/module as an external user would
use functions::math_functions;

#[test]
fn test_public_api() {
    // Can only test public functions
    let result = math_functions::add(10, 20);
    assert_eq!(result, 30);
    
    let area = math_functions::calculate_area(5, 5);
    assert_eq!(area, 25);
}

#[test]
fn test_workflow() {
    // Test a complete workflow using multiple functions
    let x = 10;
    let y = 20;
    
    let sum = math_functions::add(x, y);
    let product = math_functions::multiply(x, y);
    
    assert_eq!(sum, 30);
    assert_eq!(product, 200);
    assert!(math_functions::check_even(sum));
    assert!(math_functions::check_even(product));
}