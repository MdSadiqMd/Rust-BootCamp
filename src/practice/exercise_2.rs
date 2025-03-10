/* Exercise 2: Implementing a Trait
Define a Printable trait with a print_details method. Implement this trait for:

Your Book struct
A new Movie struct */

trait Printable {
    fn print_details(&self) -> String;
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Movie {
    title: String,
    hero: String,
}

impl Printable for Book {
    fn print_details(&self) -> String {
        format!("{}, {}, {}", self.author, self.title, self.page_count)
    }
}

impl Printable for Movie {
    fn print_details(&self) -> String {
        format!("{}, {}", self.hero, self.title)
    }
}

pub fn main() {
    let book = Book {
        title: String::from("Hello"),
        author: String::from("vooo"),
        page_count: 45,
    };
    let movie = Movie {
        title: String::from("movie title"),
        hero: String::from("hero name"),
    };

    println!("{}", book.print_details());
    print!("{}", movie.print_details());
}
