/* Exercise 4: Associated Types in Traits
Create a Collection trait with an associated type Item and methods add and get_all. Implement it for a custom Library type that stores books. */

struct Book {
    name: String,
    author: String,
}

trait Collection {
    type Item;
    fn add(&mut self, book: Self::Item);
    fn get_all(&self) -> Vec<&Self::Item>;
}

struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    fn new(name: String) -> Self {
        Library {
            name,
            books: Vec::new(),
        }
    }
}

impl Collection for Library {
    type Item = Book;

    fn add(&mut self, book: Self::Item) {
        return self.books.push(book);
    }

    fn get_all(&self) -> Vec<&Self::Item> {
        return self.books.iter().collect();
    }
}

pub fn main() {
    let mut library = Library::new(String::from("Library"));
    library.add(Book {
        name: String::from("book 1"),
        author: String::from("author 1"),
    });
    library.add(Book {
        name: String::from("book 2"),
        author: String::from("author 2"),
    });

    for book in library.get_all() {
        println!("{},{}", book.name, book.author)
    }
}
