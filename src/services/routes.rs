use rocket::response::content::Json;


// Define the "people" table model
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "people"]
struct Person {
    id: Option<i32>,
    first_name: String,
    last_name: String,
    height: i32,
}



#[get("/random")]
fn get_random_person() -> Json<Person> {
    Json(
        Person {
            id: None,
            first_name: "".to_string(),
            last_name: "".to_string(),
            height: 198
        }
    )
}