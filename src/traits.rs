use std::f64::consts::PI;

// Define a trait that describes basic shape behaviors
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Structs representing different shapes
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

// Implement the Shape trait for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

// Implement the Shape trait for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// Implement the Shape trait for Triangle
impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

// A function that accepts any type that implements the Shape trait
fn print_shape_info<T: Shape>(shape: &T) {
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}", shape.perimeter());
}

// Another version of the function using trait objects
fn print_shape_info_trait_object(shape: &dyn Shape) {
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}", shape.perimeter());
}

pub fn main() {
    // Create instances of each shape
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let triangle = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };

    // Using generic trait bounds
    println!("Using generic trait bounds:");
    print_shape_info(&circle);
    print_shape_info(&rectangle);
    print_shape_info(&triangle);

    // Using trait objects
    println!("\nUsing trait objects:");
    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle, &triangle];
    for shape in shapes {
        print_shape_info_trait_object(shape);
    }
}
