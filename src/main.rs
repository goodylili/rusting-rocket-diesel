#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;


use diesel::prelude::*;
use rocket::delete;
use rocket::get;
use rocket::post;
use rocket::put;
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;

use crate::database::establish_connection;
use crate::models::{NewStudent, Student, UpdateStudent};
use crate::schema::student::dsl::student;

mod schema;
mod database;
mod models;


#[get("/students")]
pub fn get_students() -> Json<JsonValue> {
    let connection = establish_connection();

    let students = student.load::<Student>(&connection).expect("Error loading students");

    Json(JsonValue::from(json!({
        "students": students,
    })))
}


#[delete("/students/<id>")]
pub fn delete_student(id: i32) -> Json<JsonValue> {
    let connection = establish_connection();

    diesel::delete(student.find(id)).execute(&connection).expect(&format!("Unable to find student {}", id));

    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("Student with ID {} has been deleted", id),
    })))
}


#[post("/student", format = "json", data = "<new_student>")]
pub fn create_student(new_student: Json<NewStudent>) -> Json<JsonValue> {
    let connection = establish_connection();
    let new_student = NewStudent {
        first_name: new_student.first_name,
        last_name: new_student.last_name,
        age: 17,
    };

    diesel::insert_into(crate::schema::student::dsl::student)
        .values(&new_student)
        .execute(&connection)
        .expect("Error saving new student");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Student has been created",

    })))
}

#[put("/students/<id>", data = "<update_data>")]
pub fn update_student(id: i32, update_data: Json<UpdateStudent>) -> Json<JsonValue> {
    let connection = establish_connection();

    // Use the `update` method of the Diesel ORM to update the student's record
    let _updated_student = diesel::update(student.find(id))
        .set(&update_data.into_inner())
        .execute(&connection)
        .expect("Failed to update student");

    // Return a JSON response indicating success
    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("Student {} has been updated", id),
    })))
}



fn main() {
        rocket::ignite().mount("/", routes![
        get_students,
        delete_student,
        create_student,
        update_student,
    ]).launch();
}
