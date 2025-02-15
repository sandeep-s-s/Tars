use crate::schema::tabs;
use crate::{db, models, schema};
use diesel::prelude::*;

use diesel::result::Error;
use uuid::Uuid;

use crate::models::{
    Collection, CollectionWithRequests, JsonResponse, NewCollection, NewRequest, NewTab,
    RequestObject, Requests, TabResponse, Tabs,
};
use crate::utils::helper;

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
pub fn create_request(rname: String, uuid: String) -> Result<Requests, String> {
    let request_name = rname.clone();
    use crate::schema::*;
    let connection = &mut db::establish_connection();
    let request_uuid = Uuid::new_v4().hyphenated().to_string();

    let collection: models::Collection = schema::collections::table
        .filter(schema::collections::uuid.eq(uuid))
        .select(Collection::as_select())
        .first(connection)
        .expect("Error loading collection");

    let request_object = RequestObject::new(
        rname,
        "GET".to_string(),
        "https://httpbin.org/get".to_string(),
    );

    let request_json = serde_json::to_string(&request_object)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let new_request: NewRequest = NewRequest {
        name: request_name,
        request_data: request_json,
        uuid: String::from(&request_uuid),
        collection_id: collection.id,
    };

    diesel::insert_into(requests::table)
        .values(&new_request)
        .returning(Requests::as_returning())
        .get_result(connection)
        .map_err(|e| format!("Error in creating new request: {}", e))
}

#[tauri::command]
pub async fn get_request(uuid: String) -> Requests {
    let mut connection = db::establish_connection();

    let request: models::Requests = schema::requests::table
        .filter(schema::requests::uuid.eq(uuid))
        .select(Requests::as_select())
        .first(&mut connection)
        .expect("Error loading request");

    let exists_tab = schema::tabs::table
        .select(Tabs::as_select())
        .filter(schema::tabs::requests_id.eq(request.id))
        .first(&mut connection)
        .optional()
        .expect("Error loading tab details");

    if exists_tab.is_none() {
        let new_tab_data = NewTab {
            requests_id: request.id,
            is_active: true,
            order_id: 0,
        };
        let _ = diesel::insert_into(tabs::table)
            .values(&new_tab_data)
            .execute(&mut connection);
    }

    return request;
}

#[tauri::command]
pub fn save_request(uuid: String, request: String) -> Result<Requests, String> {
    let connection = &mut db::establish_connection();

    let request_object: RequestObject =
        serde_json::from_str(&request).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    if !request_object.name.is_empty() {
        diesel::update(schema::requests::table)
            .filter(schema::requests::dsl::uuid.eq(uuid))
            .set(schema::requests::dsl::request_data.eq(request))
            .returning(Requests::as_returning())
            .get_result(connection)
            .map_err(|e| format!("Error in updating request: {}", e))
    } else {
        Err("Request name is empty".to_string())
    }
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
    if request.method.to_lowercase().as_str() == "delete" {
        return helper::send_delete_request(request).await;
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

#[tauri::command]
pub fn get_tabs() -> Vec<TabResponse> {
    let conn = &mut db::establish_connection();

    let results = schema::tabs::table
        .inner_join(schema::requests::table.on(schema::tabs::requests_id.eq(schema::requests::id)))
        .select((
            schema::tabs::dsl::id,
            schema::tabs::dsl::order_id,
            schema::tabs::dsl::is_active,
            schema::tabs::dsl::create_date,
            schema::tabs::dsl::update_date,
            schema::requests::dsl::name,
            schema::requests::dsl::uuid,
        ))
        .load::<(i32, i32, bool, String, String, String, String)>(conn);

    match results {
        Ok(rows) => rows
            .into_iter()
            .map(
                |(
                    id,
                    order_id,
                    is_active,
                    create_date,
                    update_date,
                    request_name,
                    request_uuid,
                )| TabResponse {
                    id,
                    order_id,
                    is_active,
                    create_date,
                    update_date,
                    request_name,
                    request_uuid,
                },
            )
            .collect(),
        Err(_) => {
            vec![]
        }
    }
}
