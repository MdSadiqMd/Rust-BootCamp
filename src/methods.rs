// An function inside a scope(most probably inside a function) is called method other than this there is not much difference between a function and method
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (like a static method in other languages)
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Instance method that borrows the instance immutably (`&self`)
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that modifies the instance (`&mut self`)
    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    // Method that modifies the instance (`&mut self`) and returns the mutable reference (for chaining)
    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    // Method that returns whether the rectangle is a square
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    // Consuming method (takes ownership of `self`)
    pub fn consume(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub fn main() {
    // Create a new Rectangle using an associated function
    let mut rect = Rectangle::new(30, 50);

    // Call an instance method (area)
    println!("Area: {}", rect.area());

    // Check if the rectangle is a square
    println!("Is square? {}", rect.is_square());

    // Mutate the rectangle using mutable methods
    rect.set_width(40);
    rect.set_height(40);
    println!("Updated rectangle: {:?}", rect);

    // Check if it's now a square after mutation
    println!("Is square? {}", rect.is_square());

    // Method chaining example
    rect.set_width(60).set_height(80);
    println!("After chaining: {:?}", rect);

    // Consume the rectangle, which moves ownership
    let (width, height) = rect.consume();
    println!("Consumed rectangle dimensions: {} x {}", width, height);

    // Uncommenting the next line would cause a compile-time error because `rect` has been moved
    // println!("{:?}", rect);  // ERROR
}
