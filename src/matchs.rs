pub fn main() {
    // 1. Basic Match on an Integer
    let number = 7;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime number."),
        13..=19 => println!("A teen number."),
        _ => println!("Something else."),
    }

    // 2. Match on Enums
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;
    match dir {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East => println!("Heading East!"),
        Direction::West => println!("Heading West!"),
    }

    // 3. Matching with Option<T>
    let some_number = Some(5);
    match some_number {
        Some(5) => println!("Got 5!"),
        Some(n) => println!("Got some other number: {}", n),
        None => println!("Got nothing!"),
    }

    // 4. Matching with Tuples
    let coordinates = (3, 4);
    match coordinates {
        (0, 0) => println!("At the origin!"),
        (0, y) => println!("On the y-axis at y = {}", y),
        (x, 0) => println!("On the x-axis at x = {}", x),
        (x, y) => println!("At coordinates: ({}, {})", x, y),
    }

    // 5. Matching with Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    match point {
        Point { x: 0, y } => println!("On the y-axis at y = {}", y),
        Point { x, y: 0 } => println!("On the x-axis at x = {}", x),
        Point { x, y } => println!("At point ({}, {})", x, y),
    }

    // 6. Matching with Guards
    let number = 15;
    match number {
        n if n % 2 == 0 => println!("{} is even", n),
        n if n % 3 == 0 => println!("{} is divisible by 3", n),
        _ => println!("{} is odd", number),
    }

    // 7. Using 'match' for returning values
    let number = 3;
    let result = match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Other",
    };
    println!("The result is: {}", result);

    // 8. Matching References
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {}", val),
    }

    // Alternatively, match using `ref` to get a reference
    let ref number = 42;
    match number {
        &val => println!("Got a value via ref: {}", val),
    }
}
