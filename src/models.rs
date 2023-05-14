use diesel::{prelude::*};
use serde_derive::{Deserialize, Serialize};

use crate::schema::student;

#[derive(Queryable, Serialize)]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "student"]
pub struct NewStudent<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: i32,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "student"]
pub struct UpdateStudent {
    first_name: Option<String>,
    last_name: Option<String>,
    age: Option<i32>,
}