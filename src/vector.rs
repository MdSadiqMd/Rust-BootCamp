pub fn main() {
    // Vectors are nothing but growable Array
    
    // 1. Creating a new empty vector (Vec)
    let mut numbers: Vec<i32> = Vec::new(); // or `let mut numbers = vec![];`

    // 2. Adding elements to a vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // 3. Printing the vector
    println!("Numbers: {:?}", numbers);

    // 4. Accessing elements by index
    println!("First element: {}", numbers[0]);

    // 5. Using the `get` method to safely access elements
    match numbers.get(1) {
        Some(value) => println!("Second element: {}", value),
        None => println!("No element found at index 1"),
    }

    // 6. Iterating over the vector
    for number in &numbers {
        println!("Number: {}", number);
    }

    // 7. Modifying elements by index
    numbers[2] = 40;
    println!("Modified vector: {:?}", numbers);

    // 8. Removing elements from the vector
    numbers.pop(); // Removes the last element
    println!("After popping the last element: {:?}", numbers);

    // 9. Vector length
    println!("Vector length: {}", numbers.len());

    // 10. Checking if the vector is empty
    if numbers.is_empty() {
        println!("The vector is empty");
    } else {
        println!("The vector is not empty");
    }

    // 11. Sorting the vector
    numbers.push(5);
    numbers.push(15);
    println!("Before sorting: {:?}", numbers);
    numbers.sort();
    println!("After sorting: {:?}", numbers);

    // 12. Clearing all elements in the vector
    numbers.clear();
    println!("After clearing, vector is empty: {:?}", numbers);
}
