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

diesel::table! {
    requests (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
        request_data -> Text,
        sort -> Integer,
        status -> Bool,
        create_date -> Timestamp,
        update_date -> Timestamp,
        collection_id -> Integer,
    }
}

diesel::joinable!(requests -> collections (collection_id));

diesel::allow_tables_to_appear_in_same_query!(collections, requests,);
