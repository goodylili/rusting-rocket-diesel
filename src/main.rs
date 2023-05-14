#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate diesel;

use diesel::associations::HasTable;
use diesel::prelude::*;
use rocket::delete;
use rocket::get;
use rocket::post;
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;

use crate::database::establish_connection;
use crate::models::{NewStudent, Student};
use crate::schema::student::dsl::student;

mod schema;
mod database;
mod models;
mod routes;


#[post("/student", format = "json", data = "<new_student>")]
fn create_student(mut new_student: Json<NewStudent>) -> Json<JsonValue> {
    let connection = establish_connection();
    let new_student = NewStudent {
        first_name: new_student.first_name,
        last_name: new_student.last_name,
        age: 17,
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

    let students = student.load::<Student>(&connection).expect("Error loading students");

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


#[post("/students/<id>", data = "<request>")]
fn update_student(conn: SqliteConnection, id: i32, request: Json<UpdateStudentRequest>) -> Json<JsonValue> {
    use crate::schema::student::dsl::*;

    // Build an update query based on the request body
    let update_query = diesel::update(student.find(id));
    let update_query = match request.first_name {
        Some(new_first_name) => update_query.set(first_name.eq(new_first_name)),
        None => update_query,
    };
    let update_query = match request.last_name {
        Some(new_last_name) => update_query.set(last_name.eq(new_last_name)),
        None => update_query,
    };
    let update_query = match request.age {
        Some(new_age) => update_query.set(age.eq(new_age)),
        None => update_query,
    };

    // Execute the update query
    update_query.execute(&conn);

    // Return a JSON response indicating success

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Student has been created",
    })))
}


fn main() {
    rocket::ignite().mount("/", routes![
        create_student,
        get_students,
        delete_student,
    ]).launch();
}