use std::collections::HashMap;

use crate::{db, models, schema, utils::helper};
use diesel::prelude::*;

use serde_json::Value;
use uuid::Uuid;

use crate::models::*;

use base64::{engine::general_purpose, Engine};
use reqwest::{
    Method,
    {
        multipart::{self, Form, Part},
        Client, Response,
    },
};

use std::path::PathBuf;

#[tauri::command]
pub fn create_collection(name: String) -> Collection {
    use crate::schema::*;
    let connection = &mut db::establish_connection();
    let uuid = Uuid::new_v4().hyphenated().to_string();

    let new_collection = NewCollection {
        name: name,
        uuid: String::from(&uuid),
        is_open: false,
    };
    diesel::insert_into(collections::table)
        .values(&new_collection)
        .returning(Collection::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub async fn get_collections() -> Vec<CollectionWithRequests> {
    let mut connection = db::establish_connection();

    let all_collections: Vec<Collection> = schema::collections::table
        .select(Collection::as_select())
        .load(&mut connection)
        .expect("Error in fetching all collections");

    // Load all requests
    let all_requests: Vec<Requests> = schema::requests::table
        .select(Requests::as_select())
        .load(&mut connection)
        .expect("Error in fetching all requests");

    // Group the requests per collection
    let requests_per_collection = all_requests
        .grouped_by(&all_collections)
        .into_iter()
        .zip(all_collections)
        .map(|(requests, collection)| CollectionWithRequests {
            collection,
            requests,
        })
        .collect::<Vec<CollectionWithRequests>>();

    requests_per_collection
}

#[tauri::command]
pub fn create_request(rname: String, uuid: String) -> Requests {
    use crate::schema::*;
    let connection = &mut db::establish_connection();
    let request_uuid = Uuid::new_v4().hyphenated().to_string();

    let collection: models::Collection = schema::collections::table
        .filter(schema::collections::uuid.eq(uuid)) // Add filter for uuid
        .select(Collection::as_select())
        .first(connection) // Load the first matching record
        .expect("Error loading collection"); //

    let request_json = r#"{
                    "v": "1",
                    "endpoint": "https://httpbin.org/post",
                    "name": "Get-req",
                    "params": [],
                    "headers": [],
                    "method": "GET",
                    "auth": {
                        "authType": "basic",
                        "authActive": false,
                        "username": "",
                        "password": "",
                        "token": ""
                    },
                    "preRequestScript": "",
                    "testScript": "",
                    "body": {
                        "mode": "None",
                        "formData": [],
                        "xWwwFormUrlencoded": [],
                        "rawType": "Json",
                        "raw": "",
                        "fromData": []
                    }
                    }"#;
    let new_request: NewRequest = NewRequest {
        name: rname,
        request_data: request_json.to_string(),
        uuid: String::from(&request_uuid),
        collection_id: collection.id,
    };
    diesel::insert_into(requests::table)
        .values(&new_request)
        .returning(Requests::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub async fn get_request(uuid: String) -> String {
    let mut connection = db::establish_connection();
    let request: models::Requests = schema::requests::table
        .filter(schema::requests::uuid.eq(uuid)) // Add filter for uuid
        .select(Requests::as_select())
        .first(&mut connection) // Load the first matching record
        .expect("Error loading request"); //
    return request.request_data;
}

#[tauri::command]
pub fn save_request(uuid: String, request: String) -> Requests {
    let connection = &mut db::establish_connection();

    diesel::update(schema::requests::table)
        .filter(schema::requests::dsl::uuid.eq(uuid))
        .set(schema::requests::dsl::request_data.eq(request))
        .returning(Requests::as_returning())
        .get_result(connection)
        .expect("Error in updating request")
}
#[tauri::command]
pub async fn send_request(request: String) -> Result<JsonResponse, String> {
    let request: RequestObject =
        serde_json::from_str(&request).map_err(|_| "Failed to parse request".to_string())?;

    if request.method.to_lowercase().as_str() == "get" {
        return helper::send_get_request(request).await;
    }
    if request.method.to_lowercase().as_str() == "post" {
        return helper::send_post_request(request).await;
    }
    if request.method.to_lowercase().as_str() == "put" {
        return helper::send_put_request(request).await;
    }
    return Err(String::from("Unsupported HTTP method"));
}

#[tauri::command]
pub fn toggle_collection(uuid: String) -> Collection {
    let collection_uuid = uuid.clone();
    let connection = &mut db::establish_connection();

    let collection: models::Collection = schema::collections::table
        .filter(schema::collections::uuid.eq(uuid))
        .select(Collection::as_select())
        .first(connection)
        .expect("Error loading request");

    let toggle_is_open = !collection.is_open;

    diesel::update(schema::collections::table)
        .filter(schema::collections::dsl::uuid.eq(collection_uuid))
        .set(schema::collections::dsl::is_open.eq(toggle_is_open))
        .returning(Collection::as_returning())
        .get_result(connection)
        .expect("Error in updating request")
}

#[tauri::command]
pub fn rename_collection(uuid: String, name: String) -> Collection {
    let collection_uuid = uuid.clone();
    let connection = &mut db::establish_connection();
    diesel::update(schema::collections::table)
        .filter(schema::collections::dsl::uuid.eq(collection_uuid))
        .set(schema::collections::dsl::name.eq(name))
        .returning(Collection::as_returning())
        .get_result(connection)
        .expect("Error in updating collection")
}

#[tauri::command]
pub fn rename_request(uuid: String, rname: String) -> Requests {
    let request_uuid = uuid.clone();
    let connection = &mut db::establish_connection();
    diesel::update(schema::requests::table)
        .filter(schema::requests::dsl::uuid.eq(request_uuid))
        .set(schema::requests::dsl::name.eq(rname))
        .returning(Requests::as_returning())
        .get_result(connection)
        .expect("Error in renaming request")
}