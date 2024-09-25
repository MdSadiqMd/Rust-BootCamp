// Define a Struct to represent a simple Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// Define an Enum to represent different kinds of Shapes
enum Shape {
    Circle(f64),             // Single variant holding the radius of the circle
    Triangle(u32, u32, u32), // Holds the lengths of the three sides
    Square(u32),             // Single variant for square side length
    Rectangle(Rectangle),    // Struct embedded in an enum variant
}

impl Rectangle {
    // A method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// A function to calculate the area of different shapes (Enums)
fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.1416 * radius * radius, // Area of a circle
        Shape::Triangle(a, b, c) => {
            // Using Heron's formula for triangle area
            let s = (a + b + c) as f64 / 2.0;
            (s * (s - a as f64) * (s - b as f64) * (s - c as f64)).sqrt()
        }
        Shape::Square(side) => (side * side) as f64, // Area of a square
        Shape::Rectangle(rect) => rect.area() as f64, // Area of a rectangle via method
    }
}

pub fn main() {
    // Using a Struct
    let my_rectangle = Rectangle {
        width: 10,
        height: 5,
    };
    println!("Rectangle Area (using struct): {}", my_rectangle.area());

    // Using Enums with different shapes
    let circle = Shape::Circle(4.5);
    let triangle = Shape::Triangle(3, 4, 5);
    let square = Shape::Square(4);
    let rect_enum = Shape::Rectangle(my_rectangle); // Struct in an Enum

    // Printing areas for each shape
    println!("Circle Area (using enum): {}", area(circle));
    println!("Triangle Area (using enum): {}", area(triangle));
    println!("Square Area (using enum): {}", area(square));
    println!("Rectangle Area (using enum): {}", area(rect_enum));
}
