// Function definition
// In rust Functions we can return Expressions but we ***cannot return Statements
fn main() {
    // Statements
    let x = 5; 
    
    // Expression
    let y = {
        let z = 6; 
        z + 1 // ***This Line not have a semicolon means we are returning this value
    };
    
    // Function call
    let result = add_numbers(x, y); 
    
    println!("Result: {}", result);
}


fn add_numbers(a: i32, b: i32) -> i32 { // -> i32 means we are specifiing the retrun type as 32-bit Integer
    a + b // ***This Line not have a semicolon means we are returning this value
    // (or) return a+b
    // (or) return a+b;
}
