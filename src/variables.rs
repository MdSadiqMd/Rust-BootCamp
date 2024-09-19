pub fn main() {
    let a: i8 = -5; // For Signed Integers < 0
    let b: u8 = 10; // For UnSigned Integers -Infinity to +Infinity
    let c: f64 = 4.656; // For Float Values
    let d: bool = true; // For Booleans
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
    // If we want to change a variable the we need to use mut (mutable Keyboard)

    // Strings are bit tricky in rust as they are highly volatile in changing the length at run time and we can't even decide the length as it is not integer that we set the length of bits
    //let greeting: &str = "hello world"; // &str --> fixed size,statically allocated,Immutable,stored in programs binary
    let greeting: String = String::from("hello world"); // String -->  heap-allocated,growable,mutable
    println!("greeting: {}", greeting);

    // Another Problem with strings is we cannot directly access a char in the String as We need to perform pattern matching as the string might be undefined
    let char = greeting.chars().nth(2); // Accessing an Char at Index 2
    match char {
        // Pattern Matching
        Some(c) => println!("{}", c),
        None => println!("No char at Index"),
    }
    println!("{}", char.unwrap()); // If you don't want to handle the undefined in the String (Run Time Exceptions)

    // we need to explicitly provide type to a `const` variable but in case of let the rust compiler and rust-analyzer will automatically will assign a type
    const PI: f64 = 3.14;
    println!("{PI}");

    // Shadowing in Rust
    let shadow = 10;
    let shadow = 20; // This variable is shadowing the above shadow variable
    println!("{shadow}") // 20
}