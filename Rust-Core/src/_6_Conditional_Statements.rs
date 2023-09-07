fn main() {
    let number = 42;

    // Basic if/else statement
    if number < 0 {
        println!("Number is negative");
    } else if number == 0 {
        println!("Number is zero");
    } else {
        println!("Number is positive");
    }

    // Using if as an expression (ternary-like)
    let is_positive = if number > 0 {
        true
    } else {
        false
    };
    println!("Is the number positive? {}", is_positive);

    // Pattern matching with match
    match number {
        0 => println!("Number is zero"),
        1..=100 => println!("Number is between 1 and 100 (inclusive)"),
        _ => println!("Number is either negative or greater than 100"),
    }

    // Combining conditions with logical operators
    let is_even = number % 2 == 0;
    let is_divisible_by_3 = number % 3 == 0;

    if is_even && is_divisible_by_3 {
        println!("Number is even and divisible by 3");
    } else if is_even || is_divisible_by_3 {
        println!("Number is either even or divisible by 3");
    } else {
        println!("Number is neither even nor divisible by 3");
    }
}
