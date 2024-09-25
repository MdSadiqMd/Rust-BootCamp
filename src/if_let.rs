// Define an enum for different types of messages
enum Message {
    Text(String),
    Number(i32),
    Quit,
}

pub fn main() {
    // Example 1: Handling Option
    let some_option: Option<i32> = Some(42);
    let none_option: Option<i32> = None;

    // Using if let with Option to check for Some variant
    if let Some(value) = some_option {
        println!("Got a value from some_option: {}", value);
    } else {
        println!("some_option was None");
    }

    // Trying it with a None variant
    if let Some(value) = none_option {
        println!("Got a value from none_option: {}", value);
    } else {
        println!("none_option was None");
    }

    // Example 2: Handling Result
    let success_result: Result<i32, &str> = Ok(100);
    let error_result: Result<i32, &str> = Err("Error occurred");

    // Using if let with Result to check for Ok variant
    if let Ok(value) = success_result {
        println!("Operation was successful with result: {}", value);
    }

    // Using if let with Result to handle the error case
    if let Err(err) = error_result {
        println!("Operation failed with error: {}", err);
    }

    // Example 3: Handling custom enums
    let msg1 = Message::Text(String::from("Hello, Rust!"));
    let msg2 = Message::Number(30);
    let msg3 = Message::Quit;

    // Using if let to handle the Message enum
    if let Message::Text(content) = msg1 {
        println!("Received a text message: {}", content);
    }

    // Checking for a number message
    if let Message::Number(num) = msg2 {
        println!("Received a number message: {}", num);
    }

    // Handle the Quit message without doing anything
    if let Message::Quit = msg3 {
        println!("Received a Quit message");
    }
}
