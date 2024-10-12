// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
        sort -> Integer,
        is_open -> Bool,
        status -> Bool,
        create_date -> Timestamp,
        update_date -> Timestamp,
    }
}
