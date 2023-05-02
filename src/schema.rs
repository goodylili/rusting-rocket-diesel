// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        height -> Integer,
    }
}
