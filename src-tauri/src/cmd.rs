use crate::db;
use crate::models;
use crate::schema;
use diesel::prelude::*;
use std::collections;
use uuid::Uuid;

use crate::models::*;

#[tauri::command]
pub fn create_collection(name: String) -> Collection {
    use crate::schema::*;
    let connection = &mut db::establish_connection();
    let uuid = Uuid::new_v4().hyphenated().to_string();

    let new_collection = NewCollection {
        name: name,
        uuid: String::from(&uuid),
    };
    diesel::insert_into(collections::table)
        .values(&new_collection)
        .returning(Collection::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}


// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn get_collections() -> Vec<Collection> {
use crate::schema::collections::dsl::collections;
 let mut connection = db::establish_connection();
//   let results = collections
//     .order(schema::collections::sort.asc())
//     .load::<models::Collection>(&mut connection)
//     .expect("Expect loading folders");
let results = collections
        // .filter(published.eq(1))
        // .limit(5)
        .select(Collection::as_select())
        .load(&mut connection)
        .expect("Error loading posts");
  results
}
