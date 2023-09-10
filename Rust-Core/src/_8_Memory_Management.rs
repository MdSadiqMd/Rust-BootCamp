fn main() {
    // Stack memory:
    // - Used for function call frames and local variables.
    // - Faster access but limited in size.
    // - Automatically managed (push/pop) as functions are called and return.
    let x = 5; // 'x' is allocated on the stack

    // Heap memory:
    // - Used for dynamic data that can grow or outlive the current scope.
    // - Slower access but can handle large or long-lived data.
    // - Managed by the programmer (allocation and deallocation).
    let y = Box::new(10); // 'y' points to an integer allocated on the heap

    // Ownership and borrowing ensure proper memory management.
    // Ownership and move semantics
    let a = String::from("Hello"); // 'a' owns a dynamically allocated string
    let b = a; // 'a' is moved into 'b', 'a' is no longer valid
    // println!("{}", a); // This line won't compile since 'a' is no longer valid

    // Borrowing
    let c = &b; // 'c' borrows 'b'
    println!("'c' references: {}", c);

    // Mutable borrowing
    let mut d = String::from("Mutable");
    let e = &mut d; // Mutable borrow of 'd'
    e.push_str(" Borrowing");
    println!("'e' references: {}", e);

    // Lifetime annotation
    let result;
    {
        let r1 = &b;
        result = longest(r1, e);
    }
    println!("Longest string: {}", result);
    }

    // Function that returns the longest string reference
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}