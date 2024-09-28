pub fn main() {
    // Create a String with UTF-8 text
    let mut hello = String::from("Hello, world!");

    // Print the original string
    println!("Original string: {}", hello);

    // Add UTF-8 characters (including emojis)
    hello.push_str(" 👋🌍");
    println!("String after appending: {}", hello);

    // Iterate over the string as bytes
    println!("\nIterating over string as bytes:");
    for byte in hello.as_bytes() {
        print!("{}, ", byte);
    }

    // Iterate over the string as chars (Unicode scalar values)
    println!("\n\nIterating over string as chars:");
    for ch in hello.chars() {
        println!("{}", ch);
    }

    // Accessing specific characters: note that Rust strings don't allow direct indexing
    let hello_slice = &hello[0..5]; // Slicing the string (safe for UTF-8 boundaries)
    println!("\nSliced string: {}", hello_slice);

    // Getting the length of the string
    let length = hello.len(); // Length in bytes, not characters
    println!("Length of string (in bytes): {}", length);

    // Trying to index into the string will give you an error, but here's how slicing works safely
    if let Some(first_char) = hello.chars().nth(0) {
        println!("First character: {}", first_char);
    }

    // Convert string to uppercase
    let upper = hello.to_uppercase();
    println!("\nUppercase version: {}", upper);

    // Check if the string contains a substring (UTF-8 aware)
    if hello.contains("world") {
        println!("The string contains 'world'");
    }

    // Clear the string
    hello.clear();
    println!("\nString after clearing: {}", hello);
}
