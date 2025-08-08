// Week 1: Control Flow Exercise
// This exercise covers if/else statements, loops, and pattern matching in Rust

fn main() {
    println!("=== Control Flow Exercise ===\n");

    // Exercise 1: If/Else Statements
    println!("Exercise 1: If/Else Statements");
    let number = 7;
    
    // TODO: Complete the if/else chain to check:
    // - If number is negative
    // - If number is zero
    // - If number is positive and even
    // - If number is positive and odd
    if number < 0 {
        println!("{} is negative", number);
    } else if number == 0 {
        // TODO: Print that the number is zero
    } else if number % 2 == 0 {
        // TODO: Print that the number is positive and even
    } else {
        // TODO: Print that the number is positive and odd
    }
    println!();

    // Exercise 2: Loop with Break
    println!("Exercise 2: Loop with Break");
    let mut count = 0;
    
    // TODO: Create a loop that:
    // - Increments count by 1 each iteration
    // - Prints the current count
    // - Breaks when count reaches 5
    loop {
        count += 1;
        println!("Count is: {}", count);
        // TODO: Add break condition
    }
    println!();

    // Exercise 3: While Loop
    println!("Exercise 3: While Loop");
    let mut countdown = 3;
    
    // TODO: Create a while loop that counts down from 3 to 1
    // and then prints "Liftoff!"
    while countdown > 0 {
        // TODO: Print the countdown number
        // TODO: Decrement countdown
    }
    // TODO: Print "Liftoff!"
    println!();

    // Exercise 4: For Loop
    println!("Exercise 4: For Loop");
    let array = [10, 20, 30, 40, 50];
    
    // TODO: Use a for loop to iterate through the array
    // and print each element with its index
    for (index, element) in array.iter().enumerate() {
        // TODO: Print "Element at index {index} is {element}"
    }
    println!();

    // Exercise 5: Match Expression
    println!("Exercise 5: Match Expression");
    let day = 3;
    
    // TODO: Complete the match expression to print the day of the week
    // 1 = Monday, 2 = Tuesday, ... 7 = Sunday
    // Any other number should print "Invalid day"
    let day_name = match day {
        1 => "Monday",
        2 => "Tuesday",
        // TODO: Add remaining days
        _ => "Invalid day",
    };
    println!("Day {} is {}", day, day_name);
    println!();

    // Exercise 6: Match with Multiple Patterns
    println!("Exercise 6: Match with Multiple Patterns");
    let score = 85;
    
    // TODO: Use match to determine the letter grade
    // 90-100: A, 80-89: B, 70-79: C, 60-69: D, 0-59: F
    let grade = match score {
        90..=100 => "A",
        // TODO: Add remaining grade ranges
        _ => "Invalid score",
    };
    println!("Score {} is grade {}", score, grade);
    println!();

    // Exercise 7: If Let
    println!("Exercise 7: If Let");
    let some_value = Some(10);
    
    // TODO: Use if let to extract and print the value if it exists
    if let Some(value) = some_value {
        // TODO: Print "The value is: {value}"
    }
    println!();

    // Exercise 8: Nested Loops
    println!("Exercise 8: Nested Loops");
    
    // TODO: Create nested loops to print a 3x3 multiplication table
    // Expected output:
    // 1 x 1 = 1    1 x 2 = 2    1 x 3 = 3
    // 2 x 1 = 2    2 x 2 = 4    2 x 3 = 6
    // 3 x 1 = 3    3 x 2 = 6    3 x 3 = 9
    for i in 1..=3 {
        for j in 1..=3 {
            // TODO: Print multiplication with proper formatting
            print!("{} x {} = {}\t", i, j, i * j);
        }
        // TODO: Print newline after each row
    }
    println!();

    // Bonus Exercise: FizzBuzz
    println!("Bonus: FizzBuzz (1-15)");
    
    // TODO: Implement FizzBuzz for numbers 1 to 15
    // - Print "Fizz" for multiples of 3
    // - Print "Buzz" for multiples of 5
    // - Print "FizzBuzz" for multiples of both 3 and 5
    // - Otherwise print the number
    for num in 1..=15 {
        // TODO: Implement FizzBuzz logic
    }
}
