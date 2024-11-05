use std::collections::HashMap;

pub fn main() {
    // 1. Create a new HashMap
    let mut student_grades: HashMap<&str, u8> = HashMap::new();

    // 2. Insert key-value pairs into the HashMap
    student_grades.insert("Alice", 85);
    student_grades.insert("Bob", 90);
    student_grades.insert("Charlie", 78);
    println!("Initial grades: {:?}", student_grades);

    // 3. Retrieve a value using a key
    // Using `get` returns an Option, which we need to handle
    if let Some(grade) = student_grades.get("Alice") {
        println!("Alice's grade: {}", grade);
    } else {
        println!("Alice's grade not found");
    }

    // 4. Update an existing entry
    // Here we increment Bob's grade by 5
    if let Some(grade) = student_grades.get_mut("Bob") {
        *grade += 5;
    }
    println!("Grades after updating Bob's grade: {:?}", student_grades);

    // 5. Insert or update with `entry`
    // Using `entry`, we can set a default value if the key doesn't exist
    student_grades.entry("David").or_insert(82);
    student_grades.entry("Alice").or_insert(92); // This won't change Alice's grade
    println!("Grades after using `entry`: {:?}", student_grades);

    // 6. Remove an entry
    student_grades.remove("Charlie");
    println!("Grades after removing Charlie: {:?}", student_grades);

    // 7. Iterate over keys and values
    println!("All students and grades:");
    for (student, grade) in &student_grades {
        println!("{}: {}", student, grade);
    }

    // 8. Check if a key exists
    if student_grades.contains_key("Alice") {
        println!("Alice's grade is in the HashMap.");
    }

    // 9. Advanced Operations

    // Merge another HashMap into the existing one
    let new_grades: HashMap<&str, u8> = [("Eve", 95), ("Frank", 88)].iter().cloned().collect();
    for (student, grade) in new_grades {
        student_grades.entry(student).or_insert(grade);
    }
    println!("After merging new grades: {:?}", student_grades);

    // Count frequency of items in a list using HashMap
    let fruits = ["apple", "banana", "apple", "orange", "banana", "banana"];
    let mut fruit_count: HashMap<&str, u32> = HashMap::new();

    for fruit in fruits {
        let count = fruit_count.entry(fruit).or_insert(0);
        *count += 1;
    }
    println!("Fruit count: {:?}", fruit_count);

    // 10. Clearing the HashMap
    student_grades.clear();
    println!("Grades after clearing: {:?}", student_grades);
}
