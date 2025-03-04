// mod variables;
// mod conditionals;
// mod loops;
// mod functions;
// mod ownership;
// mod references_and_borrowings;
// mod slice;
// mod structs;
// mod methods;
// mod enums;
// mod matchs;
// mod if_let;
// mod vector;
// mod utf8_strings;
// mod hashmap;
// mod panic;
// mod generics;
// mod traits;
// mod lifetimes;

enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}

impl Shape {
    fn area(self) -> f32{
        return match self {
            Shape::Circle(r) => 12.0,
            Shape::Square(l) => 15.0,
            Shape::Rectangle(l, r) => 12.0 * 15.0,
            _ => 20.0,
        }
    }
}

fn main() {
    // println!("Hello, world!");
    // variables::main();
    // conditionals::main();
    // loops::main();
    // functions::main();
    // ownership::main();
    // references_and_borrowings::main();
    // slice::main();
    // structs::main();
    // methods::main();
    // enums::main();
    // matchs::main();
    // if_let::main();
    // vector::main();
    // utf8_strings::main();
    // hashmap::main();
    // panic::main();
    // generics::main();
    // traits::main();
    // lifetimes::main();
    println!("{}", Shape::Square(12.0).area());
}

fn caluclate_area(s: Shape) -> f32 {
    return match s {
        Shape::Circle(r) => 12.0,
        Shape::Square(l) => 15.0,
        Shape::Rectangle(l, r) => 12.0 * 15.0,
        _ => 20.0,
    }
}
