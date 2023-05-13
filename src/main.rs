use rocket_codegen::routes;

mod schema;
mod database;
mod models;
mod routes;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

fn main() {
    println!("Rocket igniting...");
    rocket::ignite()
        .mount("/", routes![create_student, get_students, update_student, delete_student])
        .launch();
}