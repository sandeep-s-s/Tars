use std::collections::HashMap;

use crate::{db, models, schema};
use diesel::prelude::*;

use uuid::Uuid;

use crate::models::*;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

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
pub fn create_request(name: String, uuid: String) -> Requests {
    use crate::schema::*;
    let connection = &mut db::establish_connection();
    let request_uuid = Uuid::new_v4().hyphenated().to_string();

    let collection: models::Collection = schema::collections::table
        .filter(schema::collections::uuid.eq(uuid)) // Add filter for uuid
        .select(Collection::as_select())
        .first(connection) // Load the first matching record
        .expect("Error loading collection"); //

    let requestJson = r#"{
        "v": "1",
        "endpoint": "",
        "name": "Untitled Request",
        "params": [],
        "headers": [],
        "method": "GET",
        "auth": {
            "authType": "",
            "authActive": false,
            "username": "",
            "password": "",
            "token": ""
        },
        "preRequestScript": "",
        "testScript": "",
        "body": {
            "contentType": null,
            "body": null
        }
    }"#;
    let new_request: NewRequest = NewRequest {
        name: name,
        request_data: requestJson.to_string(),
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
pub fn send_request(request: String) -> Result<JsonResponse, String> {
    // let connection = &mut db::establish_connection();
 // Create a new HTTP client
  let client = Client::new();

    // Define headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"));

    // Define form data
    let mut form_data = HashMap::new();
    form_data.insert("key1", "value1");
    form_data.insert("key2", "value2");

    // Send the POST request
    let post_response = client
        .post("https://echo.hoppscotch.io")
        .headers(headers) // Use the same headers for POST
        .form(&form_data) // Send form data
        .send();


    // Handle the response
    match post_response {
        Ok(response) => {
            let status_code = response.status().as_u16();
            let mut response_headers = HashMap::new();

            // Collect headers into a HashMap
            for (key, value) in response.headers() {
                response_headers.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
            }

            if response.status().is_success() {
                let body = response.text().unwrap_or_else(|_| "No body".to_string());
                Ok(JsonResponse {
                    success: true,
                    message: format!("Request was successful! Response body: {}", body),
                    status_code,
                    headers: response_headers,
                })
            } else {
                Ok(JsonResponse {
                    success: false,
                    message: format!("Request failed with status: {}", response.status()),
                    status_code,
                    headers: response_headers,
                })
            }
        }
        Err(e) => {
            Err(format!("Error sending request: {}", e))
        }
    } 
 
}
