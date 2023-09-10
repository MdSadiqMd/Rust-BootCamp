use std::io;

fn main() {
    let mut input = String::new();
    
    /*This Imports the stdin() from io Package and read it uisng read_line (**works for string only) at reference of &mut in variable inout and retrun "Invalid Input" erro if given input is invalid */
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    println!("{}", input);
}
