use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Message {
    pub text: String,
}

#[derive(Deserialize)]
pub struct UserInput {
    pub username: String,
    pub age: u8,
}