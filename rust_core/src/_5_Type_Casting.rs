fn main() {
    // Define a variable of type i32 (signed integer)
    let a: i32 = 42;
    println!("Value of 'a': {}", a);

    // Type casting: Converting i32 to f64 (floating-point)
    let b: f64 = a as f64;
    println!("Value of 'b' (after casting): {}", b);

    // Type casting: Converting f64 to i32 (integer)
    let c: i32 = b as i32;
    println!("Value of 'c' (after casting): {}", c);

    // Type casting: Using the 'as' keyword with overflow
    let large_num: i64 = 1_000_000_000_000;
    let truncated_num: i32 = large_num as i32;
    println!("Value of 'truncated_num' (after casting with overflow): {}", truncated_num);

    // Type casting: Using conversion methods (e.g., to_string)
    let num_string: String = a.to_string();
    println!("Value of 'num_string': {}", num_string);

    // Type casting: Using the 'parse' method to convert a string to an integer
    let parsed_num: Result<i32, _> = num_string.parse();
    match parsed_num {
        Ok(value) => println!("Parsed integer value: {}", value),
        Err(_) => println!("Failed to parse integer from 'num_string'"),
    }
}
