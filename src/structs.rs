// Define a basic struct for a Rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing methods for the Rectangle struct
impl Rectangle {
    // Associated function (like a static method) to create a new rectangle
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that checks if the rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // A method that returns a square (use of Self keyword)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// A struct with named fields and lifetime parameters
#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
    pages: u32,
}

// Implementing methods for the Book struct
impl<'a> Book<'a> {
    // Method to print book information
    fn print_info(&self) {
        println!(
            "Title: {}, Author: {}, Pages: {}",
            self.title, self.author, self.pages
        );
    }
}

pub fn main() {
    // Create instances of Rectangle using new() and square()
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let square = Rectangle::square(20);

    // Call the area() method and print results
    println!("The area of rect1 is: {} square pixels.", rect1.area());

    // Call the can_hold() method to compare rectangles
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));

    // Demonstrating the use of a struct with lifetime
    let title = "The Rust Programming Language";
    let author = "Steve Klabnik and Carol Nichols";
    let book = Book {
        title,
        author,
        pages: 552,
    };

    // Call the method on the book instance
    book.print_info();
}
