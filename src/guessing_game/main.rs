use std::{cmp::Ordering, io};

pub fn main() {
    println!("Guess the Number");
    let secret_number = rand::random_range(1..=10);
    loop {
        println!("input the number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("please enter the number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too less"),
            Ordering::Greater => println!("more"),
            Ordering::Equal => {
                println!("you made it");
                break;
            }
        }
    }
}
