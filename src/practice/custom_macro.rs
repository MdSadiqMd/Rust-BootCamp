macro_rules! say_hello {
    () => { // when not given any argument
        println!("Hello");
    };
    ($($arg:tt)*) => { // If given an argument
        println!("Hello {}",$($arg)*);
    };
}

pub fn main() {
    say_hello!();
    say_hello!("Sadiq");
}
