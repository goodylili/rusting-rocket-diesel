#![feature(proc_macro_hygiene, decl_macro)]

use diesel::prelude::*;
use crate::database::establish_connection;
use crate::models::{NewStudent, Student};
use rocket::post;
use rocket::get;
use rocket::put;
use rocket::delete;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;
use crate::schema::student::dsl::student;


#[post("/students", format = "json", data = "<new_student>")]
fn create_student(new_student: Json<NewStudent>) -> Json<Student> {
    let connection = establish_connection();

    diesel::insert_into(student::table)
        .values(&new_student.into_inner())
        .execute(&connection)
        .expect("Error creating new student");

    let student = student::table.order(student::id.desc()).first(&connection).unwrap();
    Json(student)
}




#[put("/students/<id>", format = "json", data = "<updated_student>")]
fn update_student(id: i32, updated_student: Json<NewStudent>) -> Json<Student> {
    let connection = establish_connection();

    diesel::update(student.find(id))
        .set(updated_student.into_inner())
        .execute(&connection)
        .expect(&format!("Unable to find student {}", id));

    let student = student.find(id).first(&connection).unwrap();
    Json(student)
}





#[get("/students")]
fn get_students() -> Json<JsonValue> {
    let connection = establish_connection();

    let students = student.load::<Student>(&connection)
        .expect("Error loading students");

    Json(JsonValue::from(json!({
        "students": students,
    })))
}


#[delete("/students/<id>")]
fn delete_student(id: i32) -> Json<JsonValue> {
    let connection = establish_connection();

    diesel::delete(student.find(id))
        .execute(&connection)
        .expect(&format!("Unable to find student {}", id));

    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("Student with ID {} has been deleted", id),
    })))
}

