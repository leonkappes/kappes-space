// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        author -> Varchar,
        published -> Varchar,
    }
}
