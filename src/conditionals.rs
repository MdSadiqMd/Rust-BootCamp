pub fn main() {
    let age: bool = true;

    if age {
        println!("if: {}", age);
    } else if !age {
        println!("else if: {}", age)
    } else {
        println!("else: {}", age)
    }
}
