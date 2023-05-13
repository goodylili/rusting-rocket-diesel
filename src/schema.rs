// @generated automatically by Diesel CLI.

diesel::table! {
    student (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        age -> Integer,
    }
}
