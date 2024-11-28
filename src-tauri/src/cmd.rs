use std::collections::HashMap;

use crate::{db, models, schema};
use diesel::prelude::*;

use uuid::Uuid;

use crate::models::*;

use base64::{engine::general_purpose, Engine};
use reqwest::{blocking::Client, Method};

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

pub fn send_request1(request: String) -> Result<JsonResponse, String> {
    let request: RequestObject = serde_json::from_str(&request).expect("Failed to parse");

    let client = Client::new();
    let url = request.endpoint;

    // Query parameters
    let mut query_params = HashMap::new();
    for param in request.params {
        if param.checked.unwrap_or(false) {
            query_params.insert(param.key, param.value);
        }
    }

    // Header parameters
    let mut req_headers = reqwest::header::HeaderMap::new();
    for header in request.headers {
        if header.checked.unwrap_or(false) {
            req_headers.insert(
                reqwest::header::HeaderName::from_bytes(header.key.as_bytes())
                    .map_err(|e| String::from("An error occurred"))?,
                reqwest::header::HeaderValue::from_str(&header.value)
                    .map_err(|e| String::from("An error occurred"))?,
            );
        }
    }

    // Request Body
    let body = match request.body.mode.as_str() {
        "raw" => request.body.raw.clone(),
        "formData" => {
            let form_data: HashMap<_, _> = request
                .body
                .form_data
                .iter()
                .map(|f| (f.key.clone(), f.value.clone()))
                .collect();
            serde_urlencoded::to_string(form_data).map_err(|e| String::from(&e.to_string()))?
        }
        "x-www-form-urlencoded" => {
            let x_www_form_data: HashMap<_, _> = request
                .body
                .x_www_form_urlencoded
                .iter()
                .map(|f| (f.key.clone(), f.value.clone()))
                .collect();
            serde_urlencoded::to_string(x_www_form_data)
                .map_err(|e| String::from(&e.to_string()))?
        }
        _ => return Err(String::from("Unsupported body mode")),
    };

    let request_object = client.request(Method::GET, &url);
    if request.auth.auth_active {
        match request.auth.auth_type.as_str() {
            "bearer" => {
                let _ = request_object.bearer_auth(&request.auth.token);
            }
            "basic" => {
                let _ =
                    request_object.basic_auth(request.auth.username, Some(request.auth.password));
            }
            "noauth" => {
                // No action needed
            }
            _ => return Err(String::from("Unsupported authentication type")),
        }
    }

    // Send the request
    let response = match request.method.to_lowercase().as_str() {
        "get" => client
            .get(&url)
            .headers(req_headers)
            .query(&query_params)
            .send()
            .map_err(|e| String::from(&e.to_string()))?,
        "post" => client
            .post(&url)
            .headers(req_headers)
            .body(body)
            .query(&query_params)
            .send()
            .map_err(|e| String::from(&e.to_string()))?,
        "put" => client
            .put(&url)
            .headers(req_headers)
            .query(&query_params)
            .body(body)
            .send()
            .map_err(|e| String::from(&e.to_string()))?,
        "delete" => client
            .delete(&url)
            .headers(req_headers)
            .query(&query_params)
            .send()
            .map_err(|e| String::from(&e.to_string()))?,
        _ => return Err(String::from("Unsupported HTTP method")),
    };

    let status_code = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<HashMap<_, _>>();

    let json_response = JsonResponse {
        success: response.status().is_success(),
        message: response.text().map_err(|e| String::from(&e.to_string()))?,
        status_code,
        headers,
    };

    Ok(json_response)
}

#[tauri::command]
pub fn send_request(request: String) -> Result<JsonResponse, String> {
    let request: RequestObject = serde_json::from_str(&request).map_err(|_| "Failed to parse request".to_string())?;

    let client = Client::new();
    let url = request.endpoint;

    // Build query parameters
    let query_params: HashMap<_, _> = request.params
        .iter()
        .filter_map(|param| {
            if param.checked.unwrap_or(false) {
                Some((param.key.clone(), param.value.clone()))
            } else {
                None
            }
        })
        .collect();

    // Build header parameters
    let req_headers = request.headers.iter()
        .filter_map(|header| {
            if header.checked.unwrap_or(false) {
                let key = reqwest::header::HeaderName::from_bytes(header.key.as_bytes()).ok()?;
                let value = reqwest::header::HeaderValue::from_str(&header.value).ok()?;
                Some((key, value))
            } else {
                None
            }
        })
        .collect::<reqwest::header::HeaderMap>();

    // Build request body
    let body = match request.body.mode.as_str() {
        "raw" => request.body.raw.clone(),
        "formData" => {
            let form_data: HashMap<_, _> = request.body.form_data.iter()
                .filter_map(|f| {
                    if f.checked.unwrap_or(false) {
                        Some((f.key.clone(), f.value.clone()))
                    } else {
                        None
                    }
                })
                .collect();
            serde_urlencoded::to_string(form_data).map_err(|e| e.to_string())?
        }
        "x-www-form-urlencoded" => {
            let x_www_form_data: HashMap<_, _> = request.body.x_www_form_urlencoded.iter()
                .filter_map(|f| {
                    if f.checked.unwrap_or(false) {
                        Some((f.key.clone(), f.value.clone()))
                    } else {
                        None
                    }
                })
                .collect();
            serde_urlencoded::to_string(x_www_form_data).map_err(|e| e.to_string())?
        }
        _ => return Err("Unsupported body mode".to_string()),
    };

    let mut request_builder = match request.method.to_lowercase().as_str() {
        "get" => client.get(&url),
        "post" => client.post(&url).body(body),
        "put" => client.put(&url).body(body),
        "delete" => client.delete(&url),
        _ => return Err("Unsupported HTTP method".to_string()),
    };

    request_builder = request_builder.headers(req_headers).query(&query_params);

    if request.auth.auth_active {
        match request.auth.auth_type.as_str() {
            "bearer" => request_builder = request_builder.bearer_auth(&request.auth.token),
            "basic" => request_builder = request_builder.basic_auth(request.auth.username, Some(request.auth.password)),
            "noauth" => {}
            _ => return Err("Unsupported authentication type".to_string()),
        }
    }

    let response = request_builder.send().map_err(|e| e.to_string())?;

    let status_code = response.status().as_u16();
    let success = response.status().is_success();
    let headers = response.headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<HashMap<_, _>>();

    let message = response.text().map_err(|e| e.to_string())?;


    let json_response = JsonResponse {
        success,
        message,
        status_code,
        headers,
    };

    Ok(json_response)
}


