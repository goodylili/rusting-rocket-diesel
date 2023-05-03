
// Define the "people" table model
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "people"]
struct Person {
    id: Option<i32>,
    first_name: String,
    last_name: String,
    height: i32,
}

