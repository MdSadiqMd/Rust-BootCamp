/* Exercise 1: Basic Structs and Implementation
Create a Book struct with fields for title, author, and page count. Implement methods to:

Create a new book
Get a formatted string description
Determine if it's a long book (>300 pages) */

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

impl Book {
    fn new(title: String, author: String, page_count: u32) -> Book {
        Book {
            title,
            author,
            page_count,
        }
    }

    fn format_description(&self) -> String {
        format!(
            "\"{}\", written by {}, {} pages",
            self.title, self.author, self.page_count
        )
    }

    fn is_long_book(&self) -> bool {
        self.page_count > 300
    }
}

pub fn main() {
    let book = Book::new(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik and Carol Nichols"),
        560,
    );

    println!("{}", book.format_description());
    println!("Is it a long book? {}", book.is_long_book());
}
