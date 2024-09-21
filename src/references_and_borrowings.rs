pub fn main() {
    // The Car analogy example --> You can modify you're car as you hold ownership, but your friend can't modify it as he doesn't hold ownership but a mechanic can modify the car via `mutable` reference

    // Ownership
    let s1 = String::from("Hello"); // s1 owns the String

    // Immutable Reference
    let len = calculate_length(&s1); // Passes an immutable reference to s1
    println!("The length of '{}' is {}.", s1, len);

    // Mutable Reference
    let mut s2 = String::from("World");
    change(&mut s2); // Passes a mutable reference to s2
    println!("Changed string: {}", s2);

    // Multiple immutable references are allowed
    let r1 = &s2;
    let r2 = &s2;
    println!("Immutable references: {}, {}", r1, r2);

    // Uncommenting the following line will cause a compile-time error
    // because you cannot have a mutable reference while immutable references are in scope.
    // let r3 = &mut s2;

    // After r1 and r2 go out of scope, we can create a mutable reference.
    {
        let r3 = &mut s2;
        r3.push_str(" from Rust!");
    }

    println!("Final string: {}", s2);
}

// Function that takes an immutable reference
fn calculate_length(s: &String) -> usize {
    s.len() // We can use s but cannot modify it
}

// Function that takes a mutable reference
fn change(s: &mut String) {
    s.push_str(" there!"); // We can modify s
}
