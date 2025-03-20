use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    age: u32,
}

pub fn main() {
    let user = User {
        username: String::from("Sadiq"),
        age: 20,
    };
    let serialized_string = serde_json::to_string(&user);
    match serialized_string {
        Ok(str) => println!("{}", str),
        Err(err) => println!("{}", err),
    }
}
