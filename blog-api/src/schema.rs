// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        author -> Int8,
        content -> Text,
        content_md -> Text,
        published -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        password -> Varchar,
        permission -> Varchar,
    }
}

diesel::joinable!(posts -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
