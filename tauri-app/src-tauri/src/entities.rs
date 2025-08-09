use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub model_id: i32,
    pub text: String
}