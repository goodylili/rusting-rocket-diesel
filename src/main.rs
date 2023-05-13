#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::response::content::Json;
use rocket::Rocket;

use self::models::*;
use self::schema::student::dsl::*;

mod schema;
mod database;
mod models;
mod routes;


fn main() {}