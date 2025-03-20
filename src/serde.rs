use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    age: u32,
}

pub fn main() {
    let user = User {
        username: String::from("Sadiq"),
        age: 20,
    };

    let serialized_string = serde_json::to_string(&user).unwrap();
    println!("{:?}", serialized_string);

    let deserialized_string: User = serde_json::from_str(&serialized_string).unwrap();
    println!("{:?}", deserialized_string);
}
