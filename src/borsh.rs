use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    username: String,
    age: u32,
    v: Vec<u32>,
}

pub fn main() {
    let user = User {
        username: String::from("Sadiq"),
        age: 20,
        v: vec![1, 2, 3, 4, 5],
    };

    let mut buffer = Vec::new();
    user.serialize(&mut buffer).unwrap(); // .serialize() is inserted by the borsh trait
    println!("{:?}", buffer);

    let deserialize_user = User::try_from_slice(&mut buffer).unwrap();
    println!("{:?}", deserialize_user)
}
