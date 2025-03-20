macro_rules! say_hello {
    () => { // when not given any argument
        println!("Hello");
    };
    ($($arg:tt)*) => { // If given an argument
        println!("Hello {}",$($arg)*);
    };
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// custom macro implementation
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

pub fn main() {
    say_hello!();
    say_hello!("Sadiq");

    // difference between {:?} and {}
    let person = Person {
        name: String::from("Sadiq"),
        age: 30,
    };
    println!("{}", person); // normal printing --> we cannot print struct directly like this thus we create an custom macro implementation as above to make it print
    println!("{:?}", person); // debug print, beautifully formatted print for debugging just need to add #[derive(Debug)] in the struct
}
