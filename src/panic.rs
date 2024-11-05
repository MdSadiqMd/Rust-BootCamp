pub fn main() {
    // 1. Basic Panic
    println!("Example 1: Basic panic!");
    // This will terminate the program with an error message.
    // panic!("This is a basic panic!");

    // Uncomment the line above to see a simple panic in action.

    // 2. Panic in a Division by Zero Scenario
    println!("\nExample 2: Division by zero");
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Division result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // 3. Out-of-Bounds Access Causing a Panic
    println!("\nExample 3: Out-of-bounds access");
    let numbers = vec![1, 2, 3];
    let index = 5;
    if let Some(value) = numbers.get(index) {
        println!("Value at index {}: {}", index, value);
    } else {
        println!(
            "Error: Attempted to access index {} in a vector of length {}",
            index,
            numbers.len()
        );
    }

    // 4. Using unwrap() and expect() for Option and Result (causing panic if None or Err)
    println!("\nExample 4: Using unwrap() and expect()");
    let _some_option: Option<i32> = None;
    // Uncommenting the line below will cause a panic because some_option is None
    // println!("Unwrapped value: {}", some_option.unwrap());

    // Using expect() to provide a custom error message
    let some_result: Result<i32, &str> = Err("Something went wrong!");
    match some_result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // 5. Handling Panic with std::panic::catch_unwind
    println!("\nExample 5: Handling panic with catch_unwind()");
    use std::panic;
    let result = panic::catch_unwind(|| {
        panic!("This is a panic inside catch_unwind!");
    });

    match result {
        Ok(_) => println!("Code executed without panic"),
        Err(_) => println!("Caught a panic!"),
    }
}

// Function that divides two numbers and returns a Result
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(a / b)
}
