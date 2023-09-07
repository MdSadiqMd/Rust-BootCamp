fn main() {
    let x: i32 = 42;
    let y: u32 = 10;
    
    // Attempting to add `x` and `y` directly will result in a type mismatch error
    // Uncomment the line below to see the error message:
    // let result = x + y;

    // To mix types, you need to explicitly convert one of them to match the other's type.
    // Here, we convert `y` to `i32` before adding them.
    let result = x + y as i32;
    
    println!("Result: {}", result);

    let z: u8 = 255; // Define an unsigned 8-bit integer with the maximum value
    
    // Attempting to increment x by 1 will result in an overflow
    // Rust will panic in debug mode, preventing the program from continuing
    // Uncomment the line below to see the panic in action:
    let v = z + 1;
    
    println!("Result: {}", v);
}
