#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(decl_macro)]




use serde_json::{json, Value};

use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};

use crate::Student;

#[post("/add", data = "<student>")]
fn create(student: Json<Student>) -> Json<Student> {
    student
}



#[get("/read")]
fn read() -> Json<Value> {
    Json(json!([
        "hero 1",
        "hero 2"
    ]))
}


#[put("/update/<id>", data = "<student>")]
fn update(id: i32, student: Json<Student>) -> Json<Student> {
    student
}


#[delete("/delete/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}


fn main() {
    rocket::ignite().mount("/", routes![create, update, delete]).mount("/", routes![read]).launch();
}
