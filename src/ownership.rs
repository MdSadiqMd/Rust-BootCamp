pub fn main() {
    // Ownership means the rust keeps track of the value with respect to the variable if the variable dies the value also gets garbage collected

    // 1. Ownership basics
    let s1 = String::from("Hello"); // `s1` owns the String
    let s2 = s1; // Ownership moves to `s2`, `s1` is no longer valid
    // println!("{}", s1); // This would cause an error because `s1` is invalid now
    println!("{}", s2); // `s2` is valid and owns the string

    // 2. Clone to keep both variables valid
    let s3 = String::from("World");
    let s4 = s3.clone(); // `s4` gets a deep copy of the String, so `s3` is still valid
    println!("s3 = {}, s4 = {}", s3, s4);

    // 3. Functions and ownership
    let s5 = String::from("Ownership Transfer");
    take_ownership(s5); // Ownership of `s5` is moved into the function
    // println!("{}", s5); // Error: `s5` is no longer valid

    let x = 5;
    makes_copy(x); // For types like integers, a copy is made, not a move
    println!("x is still valid: {}", x); // `x` is still valid here, ownerships only work for the data structures are on Heap like Strings, Vectors and Hashmaps

    // 4. Return values and ownership
    let s6 = gives_ownership();
    println!("s6: {}", s6);

    let s7 = String::from("Hello");
    let s8 = takes_and_gives_back(s7); // `s7` moves, and ownership is returned to `s8`
    println!("s8: {}", s8);
}

// Takes ownership of the string
fn take_ownership(some_string: String) {
    println!("Taking ownership: {}", some_string);
}

// Makes a copy (because i32 is Copy trait)
fn makes_copy(some_integer: i32) {
    println!("Making a copy: {}", some_integer);
}

// Gives ownership to the caller
fn gives_ownership() -> String {
    let some_string = String::from("Returned Ownership");
    some_string
}

// Takes and returns ownership
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
