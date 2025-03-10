/* Exercise 5: Multiple Trait Bounds
Write a function that requires a type implementing both Display and Debug traits from the standard library. Test it with your own custom type. */
use std::fmt::{Debug, Display};

fn print_and_debug<T: Display + Debug>(item: T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
    println!("Pretty Debug: {:#?}", item);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

pub fn main() {
    let person = Person {
        name: String::from("Hello"),
        age: 50,
    };

    print_and_debug(person);
}
