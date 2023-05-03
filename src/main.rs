#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
mod schema;
mod models;
mod services;

use std::env;
use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;



fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("You've not set the DATABASE_URL environment variable");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("There's an error connecting to the database: {}", database_url))
}



// Start the Rocket application
fn main() {
    let connection = establish_connection().expect("Failed to connect to database");
    rocket::ignite()
        .manage(connection)
        .mount("/", routes![get_people, get_person, create_person, update_person, delete_person])
        .launch();
}
