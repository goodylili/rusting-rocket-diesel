use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}