
#![feature(proc_macro_hygiene, decl_macro)]



mod schema;
mod database;
mod models;
mod routes;


#[macro_use]
extern crate diesel;


use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::models::{NewStudent, Students};
use rocket::post;
use rocket::get;
use rocket::delete;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;
use rocket::{routes};

use crate::schema::student::dsl::student;




#[post("/student", format = "json", data = "<new_student>")]
fn create_student(mut new_student: Json<NewStudent>) -> Json<JsonValue> {
    let connection = establish_connection();
    let new_student = NewStudent {
        first_name: new_student.first_name,
        last_name: new_student.last_name,
        age: 17
    };

    diesel::insert_into(self::schema::student::dsl::student)
        .values(&new_student)
        .execute(&connection)
        .expect("Error saving new student");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Student has been created",

    })))
}




#[get("/students")]
fn get_students() -> Json<JsonValue> {
    let connection = establish_connection();

    let students = student.load::<Students>(&connection).expect("Error loading students");

    Json(JsonValue::from(json!({
        "students": students,
    })))
}


#[delete("/students/<id>")]
fn delete_student(id: i32) -> Json<JsonValue> {
    let connection = establish_connection();

    diesel::delete(student.find(id)).execute(&connection).expect(&format!("Unable to find student {}", id));

    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("Student with ID {} has been deleted", id),
    })))
}

fn main() {
    rocket::ignite().mount("/", routes![
        create_student,
        get_students,
        delete_student,
    ]).launch();
}
