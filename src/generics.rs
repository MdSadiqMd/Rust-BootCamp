// A generic function to find the largest element in a slice
fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut largest = &items[0];

    for item in items {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// A generic struct
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

// Implementing methods for the generic struct Pair
impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self { first, second }
    }
}

// Adding a method with constraints
impl<T: std::fmt::Display, U: std::fmt::Display> Pair<T, U> {
    fn display(&self) {
        println!("Pair: ({}, {})", self.first, self.second);
    }
}

// A generic enum
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

// A function to demonstrate the usage of the Option enum
fn get_first_element<T>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        Option::None
    } else {
        Option::Some(&items[0])
    }
}

pub fn main() {
    // Using the largest function
    let numbers = vec![34, 50, 25, 100, 65];
    let largest_number = largest(&numbers);
    println!("The largest number is {}", largest_number);

    // Using the Pair struct
    let pair = Pair::new(1, "one");
    println!("{:?}", pair);
    pair.display();

    // Using the Option enum
    let items: Vec<i32> = vec![];
    match get_first_element(&items) {
        Option::Some(value) => println!("First element: {}", value),
        Option::None => println!("The list is empty."),
    }

    let items_with_values = vec![10, 20, 30];
    match get_first_element(&items_with_values) {
        Option::Some(value) => println!("First element: {}", value),
        Option::None => println!("The list is empty."),
    }
}
